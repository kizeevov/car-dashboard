use i_slint_core::graphics::{Rgba8Pixel, SharedPixelBuffer};
use i_slint_core::model::VecModel;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::ui::MainWindow;
use crate::ui::MapAdapter;
use crate::ui::Tile;
use crate::ui::Viewport;
use slint::ComponentHandle;
use slint::Weak;

const TILE_SIZE: isize = 256;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct TileCoordinate {
    z: u32,
    x: isize,
    y: isize,
}

struct World {
    loaded_tiles: BTreeMap<TileCoordinate, slint::Image>,
    loading_tiles: BTreeMap<TileCoordinate, Pin<Box<dyn Future<Output = slint::Image>>>>,
    osm_url: String,
    zoom_level: u32,
    visible_height: f64,
    visible_width: f64,
    offset_x: f64,
    offset_y: f64,
}

impl World {
    fn new() -> Self {
        World {
            osm_url: std::env::var("OSM_TILES_URL")
                .unwrap_or("https://tile.openstreetmap.org".to_string()),
            loaded_tiles: Default::default(),
            loading_tiles: Default::default(),
            zoom_level: 1,
            visible_height: 0.,
            visible_width: 0.,
            offset_x: 0.,
            offset_y: 0.,
        }
    }

    fn set_zoom_level(&mut self, zoom_level: u32, ox: f64, oy: f64) {
        if self.zoom_level != zoom_level {
            self.loaded_tiles.clear();
            self.loaded_tiles.clear();
            let exp2 = f64::exp2(zoom_level as f64 - self.zoom_level as f64);
            self.offset_x += ox;
            self.offset_y += oy;
            self.offset_x *= exp2;
            self.offset_y *= exp2;
            self.offset_x -= ox;
            self.offset_y -= oy;
            self.zoom_level = zoom_level;
            self.reset_view();
        }
    }

    fn reset_view(&mut self) {
        let m = 1 << self.zoom_level;
        let min_x = (self.offset_x / TILE_SIZE as f64).floor() as isize;
        let min_y = (self.offset_y / TILE_SIZE as f64).floor() as isize;
        let max_x =
            (((self.offset_x + self.visible_width) / TILE_SIZE as f64).ceil() as isize + 1).min(m);
        let max_y =
            (((self.offset_y + self.visible_height) / TILE_SIZE as f64).ceil() as isize + 1).min(m);
        // remove tiles that is too far away
        const KEEP_CACHED_TILES: isize = 10;
        let keep = |coord: &TileCoordinate| {
            coord.z == self.zoom_level
                && (coord.x > min_x - KEEP_CACHED_TILES)
                && (coord.x < max_x + KEEP_CACHED_TILES)
                && (coord.y > min_y - KEEP_CACHED_TILES)
                && (coord.y < max_y + KEEP_CACHED_TILES)
        };
        self.loading_tiles.retain(|coord, _| keep(coord));
        self.loaded_tiles.retain(|coord, _| keep(coord));

        let client = reqwest::Client::new();

        for x in min_x..max_x {
            for y in min_y..max_y {
                let coord = TileCoordinate {
                    z: self.zoom_level,
                    x,
                    y,
                };
                if self.loaded_tiles.contains_key(&coord) {
                    continue;
                }
                self.loading_tiles.entry(coord).or_insert_with(|| {
                    let url = format!("{}/{}/{}/{}.png", self.osm_url, coord.z, coord.x, coord.y);
                    let client = client.clone();
                    Box::pin(async move {
                        let response = client
                            .get(&url)
                            .header("User-Agent", "Slint Maps example")
                            .send()
                            .await;
                        let response = match response {
                            Ok(response) => response,
                            Err(err) => {
                                eprintln!("Error loading {url}: {}", err);
                                return slint::Image::default();
                            }
                        };
                        if !response.status().is_success() {
                            eprintln!("Error loading {url}: {:?}", response.status());
                            return slint::Image::default();
                        }

                        let image =
                            match image::load_from_memory(&mut response.bytes().await.unwrap()) {
                                Ok(image) => image,
                                Err(err) => {
                                    eprintln!("Error reading {url}: {}", err);
                                    return slint::Image::default();
                                }
                            };

                        let image = image
                            .resize(
                                TILE_SIZE as u32,
                                TILE_SIZE as u32,
                                image::imageops::FilterType::Nearest,
                            )
                            .into_rgba8();
                        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                            image.as_raw(),
                            image.width(),
                            image.height(),
                        );
                        slint::Image::from_rgba8(buffer)
                    })
                });
            }
        }
    }

    fn poll(&mut self, context: &mut Context, changed: &mut bool) {
        self.loading_tiles.retain(|coord, future| {
            let image = future.as_mut().poll(context);
            match image {
                Poll::Ready(image) => {
                    self.loaded_tiles.insert(*coord, image);
                    *changed = true;
                    false
                }
                Poll::Pending => true,
            }
        })
    }
}

pub struct MapState {
    world: RefCell<World>,
    main_window: Weak<MainWindow>,
    poll_handle: RefCell<Option<slint::JoinHandle<()>>>,
}

impl MapState {
    fn do_poll(self: Rc<Self>) {
        if let Some(handle) = self.poll_handle.take() {
            handle.abort();
        }
        self.refresh_model();
        slint::spawn_local(async move {
            std::future::poll_fn(|context| {
                let mut changed = false;
                self.world.borrow_mut().poll(context, &mut changed);
                if changed {
                    self.refresh_model();
                }
                if self.world.borrow().loading_tiles.is_empty() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;
        })
        .unwrap();
    }

    fn refresh_model(&self) {
        let vec = VecModel::from(
            self.world
                .borrow()
                .loaded_tiles
                .iter()
                .map(|(coord, image)| Tile {
                    tile: image.clone(),
                    x: (coord.x * TILE_SIZE) as f32,
                    y: (coord.y * TILE_SIZE) as f32,
                })
                .collect::<Vec<Tile>>(),
        );

        let binding = self.main_window.unwrap();
        let map_adapter = binding.global::<MapAdapter>();
        map_adapter.set_tiles(slint::ModelRc::new(vec));
    }

    fn set_viewport_size(&self) {
        let world = self.world.borrow();
        let zoom = world.zoom_level;

        let binding = self.main_window.unwrap();
        let map_adapter = binding.global::<MapAdapter>();

        map_adapter.set_zoom(zoom as _);
        let world_size = (TILE_SIZE * (1 << zoom)) as f32;
        map_adapter.set_viewport(Viewport {
            ox: -world.offset_x as f32,
            oy: -world.offset_y as f32,
            width: world_size,
            height: world_size,
        });
    }
}

pub fn setup(main_window: &MainWindow) -> Rc<MapState> {
    let main_window = main_window.as_weak();

    let state = Rc::new(MapState {
        world: RefCell::new(World::new()),
        main_window,
        poll_handle: RefCell::new(None),
    });

    let binding = state.main_window.unwrap();
    let map_adapter = binding.global::<MapAdapter>();

    let state_weak = Rc::downgrade(&state);
    map_adapter.on_flicked(move |ox, oy| {
        let state = state_weak.upgrade().unwrap();
        let mut world = state.world.borrow_mut();
        let binding = state.main_window.unwrap();

        world.offset_x = -ox as f64;
        world.offset_y = -oy as f64;
        world.visible_width = binding.get_map_visible_width() as f64;
        world.visible_height = binding.get_map_visible_height() as f64;
        world.reset_view();
        drop(world);
        state.do_poll();
    });

    let state_weak = Rc::downgrade(&state);
    map_adapter.on_zoom_changed(move |zoom| {
        let state = state_weak.upgrade().unwrap();
        let mut world = state.world.borrow_mut();
        let main_window = state.main_window.unwrap();

        world.visible_width = main_window.get_map_visible_width() as f64;
        world.visible_height = main_window.get_map_visible_height() as f64;
        let (vw, vh) = (world.visible_width, world.visible_height);
        world.set_zoom_level(zoom as _, vw / 2., vh / 2.);
        drop(world);
        state.set_viewport_size();
        state.do_poll();
    });

    let state_weak = Rc::downgrade(&state);
    map_adapter.on_zoom_in(move |ox, oy| {
        let state = state_weak.upgrade().unwrap();
        let mut world = state.world.borrow_mut();
        let main_window = state.main_window.unwrap();
        let z = (world.zoom_level + 1).min(19);

        world.visible_width = main_window.get_map_visible_width() as f64;
        world.visible_height = main_window.get_map_visible_height() as f64;
        world.set_zoom_level(z as _, ox as f64, oy as f64);
        drop(world);
        state.set_viewport_size();
        state.do_poll();
    });

    let state_weak = Rc::downgrade(&state);
    map_adapter.on_zoom_out(move |ox, oy| {
        let state = state_weak.upgrade().unwrap();
        let mut world = state.world.borrow_mut();
        let main_window = state.main_window.unwrap();
        let z = (world.zoom_level - 1).max(1);

        world.visible_width = main_window.get_map_visible_width() as f64;
        world.visible_height = main_window.get_map_visible_height() as f64;
        world.set_zoom_level(z as _, ox as f64, oy as f64);
        drop(world);
        state.set_viewport_size();
        state.do_poll();
    });

    {
        let state = state.clone();
        slint::spawn_local(async move {
            let mut world = state.world.borrow_mut();
            let main_window = state.main_window.unwrap();

            world.visible_width = main_window.get_map_visible_width() as f64;
            world.visible_height = main_window.get_map_visible_height() as f64;
            world.reset_view();
            drop(world);
            state.set_viewport_size();
            state.clone().do_poll();
        })
        .unwrap();
    }

    state
}

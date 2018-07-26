#![recursion_limit="128"]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate log;
extern crate rand;

mod ecs;
mod boid;
mod simulation;
mod flocks;
mod point;
mod vector;
mod obsticle;
mod range;
mod resize_service;


use yew::prelude::*;
use yew::services::TimeoutService;
use yew::services::timeout::TimeoutTask;
use simulation::Simulation;
use simulation::SimConfig;
use stdweb::web::{IParentNode, document};
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::RenderingContext;
use stdweb::web::html_element::CanvasElement;
use stdweb::unstable::TryInto;
use std::time::Duration;
use resize_service::{ResizeService, ResizeTask};


use stdweb::web::HtmlElement;
use stdweb::web::IHtmlElement;

use range::RangePicker;
use point::Point;

pub struct BoidsModel {
    simulation: Simulation,
    context: Option<CanvasRenderingContext2d>,
    editing_mode: EditingMode,
    timeout_service: TimeoutService,
    timeout_task: Option<TimeoutTask>,
    resize_task: ResizeTask,
    link: ComponentLink<BoidsModel>
}

pub enum Msg {
    SetUp,
    Tick,
    AddRandomBoid,
    AddRandomObsticle,
    Clear,
    Resize,
    UpdateMaxSpeed(f64),
    UpdateFlockingAffinity(f64),
    UpdateFlockingDistance(f64),
    UpdateAccelerationDampingFactor(f64),
    UpdateObsticleAvoidanceFactor(f64),
    UpdateObsticleAvoidanceRange(f64),
    Clicked(ClickEvent),
    SetEditingMode(EditingMode)
}

#[derive(PartialEq)]
pub enum EditingMode {
    AddBoid,
    AddObsticle,
    Remove,
    None
}

impl Component for BoidsModel {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<BoidsModel>) -> Self {
        let mut timeout_service = TimeoutService::new();
        let start_rendering = link.send_back(|_| Msg::SetUp );
        let task = timeout_service.spawn(Duration::from_millis(16), start_rendering);
        let config = SimConfig::default();
        let mut simulation = Simulation::new(config);
        simulation.populate_test();

        let mut resize_service = ResizeService::new();

        BoidsModel {
            simulation,
            context: None,
            editing_mode: EditingMode::None,
            timeout_service,
            timeout_task: Some(task),
            resize_task: resize_service.register(link.send_back(|_| Msg::Resize)),
            link
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            SetUp => {
                if self.context.is_none() {

                    self.update(Resize);

                    let canvas: CanvasElement = document().query_selector("#boids_canvas")
                        .unwrap()
                        .expect("should find the boids canvas")
                        .try_into()
                        .expect("boids canvas should be a canvas");
                        self.context = Some(CanvasRenderingContext2d::from_canvas(&canvas).expect("should get rendering context from canvas"));                
                }
                self.update(Tick);
                return true;
            }
            Resize => {
                let container: HtmlElement = document().query_selector("#canvas_containing_div").unwrap().unwrap().try_into().unwrap();
                self.simulation.config.dimensions.width = container.offset_width() as f64;
                self.simulation.config.dimensions.height = container.offset_height() as f64 - 4.0; // offset that is needed to maintain initial height?
                return true;
            }
            Tick => {

                
                // set callback for next frame
                let tick = self.link.send_back(|_| Msg::Tick );
                let task = self.timeout_service.spawn(Duration::from_millis(15), tick);
                self.timeout_task = Some(task);

                if let Some(ref mut context) = self.context {
                    // draw the current state
                    self.simulation.draw(context);
                }
                // calculate the next state
                // By the time this finishes, the callback should be ready to render.
                // Bla bla blah, don't tie your rendering cycle to your state calculation... whatever
                self.simulation.tick();
            }
            AddRandomBoid => {
                self.simulation.add_random_boid();
            }
            AddRandomObsticle => {
                self.simulation.add_random_obsticle();
            }
            Clear => {
                self.simulation.clear()
            }
            UpdateMaxSpeed(speed) => {
                self.simulation.config.max_speed = speed;
                return true;
            }
            UpdateFlockingAffinity(affinity) => {
                self.simulation.config.flocking_affinity = affinity;
                return true;
            }
            UpdateFlockingDistance(distance) => {
                self.simulation.config.flocking_distance = distance;
                return true;
            }
            UpdateAccelerationDampingFactor(factor) => {
                self.simulation.config.acceleration_damping_factor = factor;
                return true;
            }
            UpdateObsticleAvoidanceFactor(factor) => {
                self.simulation.config.obsticle_avoidance_factor = factor;
                return true;
            }
            UpdateObsticleAvoidanceRange(range) => {
                self.simulation.config.obsticle_avoidance_range = range;
                return true;
            }
            Clicked(click_event) => {
                let position = Point::from(click_event);
                match self.editing_mode {
                    EditingMode::AddBoid => {
                        self.simulation.add_boid_at_position(position)
                    }
                    EditingMode::AddObsticle => {
                        self.simulation.add_obsticle(position)
                    }
                    EditingMode::Remove => {
                        // TODO
                        self.simulation.remove_near_point(position)
                    }
                    EditingMode::None => {}
                }
            }
            SetEditingMode(editing_mode) => {
                self.editing_mode = editing_mode;
                return true
            }
        }
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<BoidsModel> for BoidsModel {
    fn view(&self) -> Html<Self> {

        html! {
            <div>
                <div id="canvas_containing_div", style="margin:0",>
                    <canvas 
                        id="boids_canvas",
                        width=self.simulation.config.dimensions.width,
                        height=self.simulation.config.dimensions.height,
                        onclick=|e| Msg::Clicked(e),
                    />
                </div>
                <div>
                    <button onclick=|_| Msg::AddRandomBoid, > {"Add Random Boid"} </button>
                    <button onclick=|_| Msg::AddRandomObsticle, > {"Add Random Obsticle"} </button>
                    <button onclick=|_| Msg::Clear, > {"Clear"} </button>
                    
                    <div>
                        <label> {"Mouse Actions"} </label>
                        <button
                            onclick=|_| Msg::SetEditingMode(EditingMode::None),
                            disabled=self.editing_mode==EditingMode::None,
                        >
                             {"Nothing"}
                        </button>
                        <button 
                            onclick=|_| Msg::SetEditingMode(EditingMode::AddBoid),
                            disabled=self.editing_mode==EditingMode::AddBoid,
                        > 
                            {"Add Boid"}
                        </button>
                        <button 
                            onclick=|_| Msg::SetEditingMode(EditingMode::AddObsticle),
                            disabled=self.editing_mode==EditingMode::AddObsticle,
                        > 
                            {"Add Obsticle"} 
                        </button>
                        <button 
                            onclick=|_| Msg::SetEditingMode(EditingMode::Remove),
                            disabled=self.editing_mode==EditingMode::Remove,
                        > 
                            {"Remove"}
                        </button>
                    </div>

                    <RangePicker:
                        title="Max Speed",
                        value=&self.simulation.config.max_speed,
                        min=1.0,
                        max=20.0,
                        callback = |val| Msg::UpdateMaxSpeed(val),
                    />
                    <RangePicker:
                        title="Flocking Affinity",
                        value=&self.simulation.config.flocking_affinity,
                        min=0.0,
                        max=10.0,
                        step=0.01,
                        callback = |val| Msg::UpdateFlockingAffinity(val),
                    />
                    <RangePicker:
                        title="Flocking Distance",
                        value=&self.simulation.config.flocking_distance,
                        min=50.0,
                        max=200.0,
                        callback = |val| Msg::UpdateFlockingDistance(val),
                    />
                    <RangePicker:
                        title="Acceleration Damping Factor",
                        value=&self.simulation.config.acceleration_damping_factor,
                        min=1.0,
                        max=10.0,
                        callback = |val| Msg::UpdateAccelerationDampingFactor(val),
                    />
                    <RangePicker:
                        title="Obsticle Avoidance Factor",
                        value=&self.simulation.config.obsticle_avoidance_factor,
                        min=0.0,
                        max=15.0,
                        callback = |val| Msg::UpdateObsticleAvoidanceFactor(val),
                    />
                    <RangePicker:
                        title="Obsticle Avoidance Range",
                        value=&self.simulation.config.obsticle_avoidance_range,
                        min=50.0,
                        max=200.0,
                        callback = |val| Msg::UpdateObsticleAvoidanceRange(val),
                    />
                </div>
            </div>
        }
    }
}
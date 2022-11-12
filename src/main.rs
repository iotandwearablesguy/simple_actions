use gdk::Display;
use gio::SimpleAction;
use glib::clone;
use gtk::{
 CssProvider, StyleContext, gio, glib, Application, ApplicationWindow, Button, Label, prelude::*, gdk,
};
use rand::Rng;
use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;
const APP_ID: &str = "org.gtk_rs.Actions3";
#[derive(Debug, Deserialize, Serialize)]

pub struct Answer {
        citya: String,
        statea: String,
        cityb: String,
        stateb: String,
        nau_mi_distance: f64,
}
impl Default for Answer {
    fn default() -> Self {
        Self { citya: Default::default(), cityb: Default::default(), statea: Default::default(), stateb: Default::default(), nau_mi_distance: Default::default(),}
    }
    }

//Setting up the csv file structure
/// This struct *is* [`CityData`]!
#[derive(Debug, Deserialize, Serialize)]
struct CityData {
    id: i32,
    city: String,
    city_ascii: String,
    state_id: String,
    state_name: String,
    county_name: String,
    lat: f64,
    lng: f64,
    population: f64,
}
#[allow(unused_variables)]
#[allow(unused_mut)]

#[allow(unused_variables)]
#[allow(unused_mut)]

/// Main binary function, responsible for:
/// * Calling command line parsing logic
/// * Setting up configuration
/// * Calling the run function in lib.rs
/// * Handling the error if the above returns an error
/// 
/// Calls out for the WASM logic from the base template
/// look for the LICENSE the folder
/// 
/// This fn *is* [`main`]! 
fn main() {
      simple_actions::run();
        println!(
        "The answer to the ultimate question is {}",
        simple_actions::answer()
    );
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
/// This fn *is* [`load_css`]!
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
/// This struct *is* [`build_ui`]!
fn build_ui(app: &Application) {
    let original_state = 0;
    let label = Label::builder()
        .label(&format!("Counter: {original_state}"))
        .build();
        label.set_widget_name("label-1");
    let label2 = Label::builder()
        .label("Two Random Cities")
        .build();
        label2.set_widget_name("label-2");
    // Create a button with label
    let button = Button::builder().label("Two Random Cities").build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Activate "win.count" and pass "1" as parameter
        let parameter = 1;
        button
            .activate_action("win.count", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
    });
    button.set_widget_name("button");
    //adding second button for feedback

    let button_1 = Button::builder().label("Hello World").build();
    button_1.connect_clicked(move |_| println!("Hello World"));
    
    //adding third button for Quit

    let button_2 = Button::builder().label("Quit").build();
       
    let button_3 = Button::builder().label("Hello again").build();
    button_3.connect_clicked(move |_| println!("Hello World Again"));
    button_3.set_widget_name("button-3");
        button_1.add_css_class("destructive-action");
        button_2.add_css_class("suggested-action");
    //adding third button for Quit

    let button_4 = Button::builder().label("Close").build();
button_4.set_widget_name("button-4");
   
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    // Create a window, set the title and add `gtk_box` to it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Distance Between Random Cities")
        //.opacity(0.50)
        .width_request(360)
        .height_request(360)
        .child(&grid)
       // .child(&gtk_box)
        .build();
        grid.attach(&label2, 0, 0, 2, 1);
        grid.attach(&label, 0, 1, 3, 2);
        grid.attach(&button, 0, 5, 2, 1);
        grid.attach(&button_1, 1, 3, 1, 1);
        grid.attach(&button_2, 0, 3, 1, 1);
        grid.attach(&button_3, 0, 4, 1, 1);
        grid.attach(&button_4, 1, 4, 1, 1);
    // Add action "count" to `window` taking an integer as parameter
    let action_count = SimpleAction::new_stateful(
        "count",
        Some(&i32::static_variant_type()),
        &original_state.to_variant(),
    );
    action_count.connect_activate(clone!(@weak label => move |action, parameter| {
        // Get state
        let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<i32>()
            .expect("The variant needs to be of type `f64`.");

        // Get parameter
        let parameter = parameter
            .expect("Could not get parameter.")
            .get::<i32>()
            .expect("The variant needs to be of type `f64`.");

        // Increase state by parameter and store state
        state += parameter;
        action.set_state(&state.to_variant());
        let answer = Answer::default();
    let myanswer = greatcircle(answer) ;
    // If an error occurs print error otherwise pass the pointer to the file.

 label.set_label(&format!("{:#?} {}", myanswer , state));
      }));
    window.add_action(&action_count);
    button_2.connect_clicked(clone!(@weak window => move |_| window.destroy()));
    button_4.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    // Present window
    window.present();
    }
/// This fn *is* [`greatcircle`]!
#[allow(unused_variables)]
    fn greatcircle(answer: Answer) -> Result<Answer, Box<dyn Error>> {
    let path = "/Users/terranceoshea/Documents/rust_codes/simple_actions/src/uscities.csv";
        //setting up the rng variable to select cities
    let mut rng = rand::thread_rng();
        
    //refer1&2 are the reference ids of the randomly chosen cities
    let refer1= rng.gen_range(1..30000);
    let refer2= rng.gen_range(1..30000);

    //setting the vars for the great circle calculation
    let mut lat1 = 0.00000;
    let mut lat2 = 0.00000;
    let mut long1 = 0.00000;
    let mut long2 = 0.00000;

    //set the vecs of bytes to read into for instances of the city data
    // To Do clean this up.
    let bytes = vec![0, 159];
    let bytes2 = vec![0, 159];
    let bytes3 = vec![0, 159];
    let bytes4 = vec![0, 159];
    let mut city1 =  String::from_utf8(bytes);
    let mut city2 = String::from_utf8(bytes2); 
    let mut state1 = String::from_utf8(bytes3);
    let mut state2 = String::from_utf8(bytes4);
    
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve and print header record
    let headers = reader.headers()?;

    println!("{:?}", headers);
    //Put the city data into records through deserializing the csv
    for result in reader.deserialize() {
        let record: CityData = result?;

        //does it match refer1
        if record.id == refer1 {
             lat1=  record.lat;
             long1 = record.lng;
             city1 = Ok(record.city);
             state1= Ok(record.state_id);
        };
        //does it match refer2
        if record.id == refer2 {
            lat2 =  record.lat;
            long2 = record.lng;
            city2 = Ok(record.city_ascii);
            state2=Ok(record.state_name);
        };
    }  

    //great circle calculation
    let earth_radius_kilometer = 6371.0_f64;
    let rlat1 = lat1.to_radians();
    let rlat2 = lat2.to_radians();

    let delta_latitude = (lat1 - lat2).to_radians();
    let delta_longitude = (long1 - long2).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + rlat2.cos() * rlat1.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;
    let miles_distance = distance * 0.621371;
    let nautical_miles_distance = miles_distance / 1.1;
    let city1= city1?;
    let city2= city2?;
    let state1= state1?;
    let state2= state2?;
    let answer = Answer {
            citya: city1,
            statea: state1,
            cityb: city2,
            stateb: state2,
            nau_mi_distance: nautical_miles_distance,
    };
   Ok(answer)
}




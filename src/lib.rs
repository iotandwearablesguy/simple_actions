//! # Simple_Actions
//! 
//! simpleActions App© 2022 Terrance O'Shea
//! terry.oshea@me.com
//! 
//! `simple_actions` is a functional code that creates a 
//! window with several buttons used to demonstrate RUST GUI's
//! 
//! # Acknowledgements:
//! 
//! * The Gui's are made with gtk4
//! 
//! * The app chooses two cities at random from a
//! csv file, then calculates the distance between the cities.
//! 
//! * The database of cities is thanks to Pareto Software, LLC,
//!  the owner of Simplemaps.com. The folder simplemaps_uscities_basicv1 
//!  contains their license and a full version of their spreadsheet and csv file.
//!
//!  # LICENSE
//! 
//! O'Shea-Dishongh No Harm License© 2022 Terrance O'Shea & Katherine Dishongh
//! 
//! <http://github.com/terryo-iotandwearablesguy>
//! 
//! Subject to the following conditions:
//! 
//!  1. That the below copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//! 
//!  2. That any use of the Software under this license never be used for harmful purposes, permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction,including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to
//!  do so subject to the aforementioned restrictions.
//! 
//! "Harmful purposes" as used herein means to not harm destroy or otherwise restrict humans, animals, or the environment. "Harmful purposes" includes, inter alia, the creation of weaponry, the guidance of weapons, limitation of food and services, damaging, contaminating or otherwise hazardous to the environmnet in any form, and/or restricting the freedom or movement of people.
//! 
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE
//! 
//! 
//! 
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

///  # SimpleActions App Approach
/// -------------------------------
///  1. Main function begins here which calls for the UI creation.
/// 
///  2. build_ui uses the grid layout method from gtk to build the GUI.
///     
///  2a. build_ui call the style.css file for the colors and font styles in the app.
///  
///  2b. build_ui makes "Two Random Cities" button active.
///  
///  2c. "Two Random Cities" is a connected_active button from gtk4 using the module of the same name.
///  
/// 3. connect_active call on the greatcircle function.
/// 
/// 3a. fn greatcircle call on the rand crate to generate two random numbers
/// 
/// 3b. fn greatcircle next reads the comma separated values file uscities matching the city_id to the random numbers generated.
///
/// 3c. the data for these two selected cities is stored in the struct CityData
/// 
/// 3d. fn greatcircle then returns the two cities with their respective states and the distance between the in the struct answer.
/// 
///  4. The struct answer is passed back to build_ui, and displayed on the screen as text in the variable label.
/// 
///  4a. build_ui monitors the buttons labeled "Quit" and "Close" which both kill the app.
///
///  4b. the "Hello World" button prints the test 'Hello World' in the terminal window.
/// 
///  4c. the "Hello Again" button prints the test 'Hello World Again' in the terminal window.
/// 
///  4d. build_ui then loops back to allow a second choice.
/// 
/// 


#[wasm_bindgen]
pub fn main() {
        run();
        println!(
        "The answer to the ultimate question is {}",
        answer());
}
/// # SimpleActions App Initializing Functions
/// ----------------------------------------
///  1. During the initial execution of the application the wasm-bingen runtime
///     is used to generate a /Douglas Adams/ Easter Egg printing "Any Douglas Adam fan knows the answer to the following:The answer to the ultimate question is 42" in the terminal window.
///  
/// 2. The struct variable 'headers' prints to the terminal window because once the headers from the csv file are read there is no use fo them.  But they must be read to move the iterator along. 
///     Since rust doesn't allow unused variables they are printed out to the terminal window with each call to fn greatcircle.  It's on the TO DO list to find a better method.
/// 
/// 
struct Answer {
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
#[allow(unused_variables)]
#[allow(unconditional_recursion)]
fn greatcircle(answer: Answer) {
    let answer = Answer::default();
    let myanswer = greatcircle(answer) ;
}
pub fn build_ui() {
gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
}
pub struct CityData {
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
pub fn run() {
    set_panic_hook();
    // Program logic here
    println!("Any Douglas Adam fan knows the answer to the following:");
}

/// Improve error messages in the browser when running as WebAssembly.
/// For more details see
/// https://github.com/rustwasm/console_error_panic_hook#readme
fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

pub fn answer() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(answer(), 42);
    }
}


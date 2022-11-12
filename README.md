<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="cmrust1.jpg" alt="Project logo"></a>
</p>

<h3 align="center">Simple Actions</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()

</div>

---

<p align="center">A GUI driven simple demo in RUST that reads a CSV file of all the US Cities and their LAT/LONGs. Choosing two random cities each time it's run from the list of cities in the CVS file. The distance between those two cities is calculated using the great circle method and then display through GTK dialog box with various buttons.
    <br> 
</p>

## 🧐 About <a name = "about"></a>

_Cavaet_ : Those viewing this repo for the first time may be horrified by the color choices in the css.  As the author I am not of an artistic nature and need to correct the css for better visual appeal.

### How it works

  1. Main function begins here which calls for the UI creation.
 
  2. build_ui uses the grid layout method from gtk to build the GUI.
    
    a. build_ui call the style.css file for the colors and font styles in the app.
    
    b. build_ui makes "Two Random Cities" button active.
    
    c. "Two Random Cities" is a connected_active button from gtk4 using the module of the same name.

 3. connect_active call on the greatcircle function.
 
  a. fn greatcircle call on the rand crate to generate two random numbers

  b. fn greatcircle next reads the comma separated values file uscities matching the city_id to the random numbers generated.

  c. the data for these two selected cities is stored in the struct CityData
  
  d. fn greatcircle then returns the two cities with their respective states and the distance between the in the struct answer.

 4. The struct answer is passed back to build_ui, and displayed on the screen as text in the variable label.
 
  a. build_ui monitors the buttons labeled "Quit" and "Close" which both kill the app.

  b. the "Hello World" button prints the test 'Hello World' in the terminal window.

  c. the "Hello Again" button prints the test 'Hello World Again' in the terminal window.

  d. build_ui then loops back to allow a second choice.

## Features

The demo is built using the *container* method, where each block of the GUI is built within a *container*.


### Build
  
Build using the "cargo run" command.

### Docs
  
Using "cargo doc" generates the documentation in the .../src/target/docs directory.


### Debug
  
Automatically sets up debugging on the MacOS using VScode and LLDB Code extensions.

#### Prerequisites

Need Vscode with extensions: 
    - rust-analyzer
    - code lldb

## ⛏️ Built Using <a name = "built_using"></a>

- [vscode](https://www.vscode.com)
- [rust](https://www.rustlang.org)

## ⛏️ Built Using <a name = "built_using"></a>

- [rust](https://www.rustlang.org)
- [simplemaps.com csv file](https://simplemaps.com/data/us-cities)
- [csv crate from Burnt Sushi @BurntSushi](https://github.com/BurntSushi)
- [rng crate from @Rand-Rust](https://github.com/rust-random)
- 

## ✍️ Author <a name = "author"></a>

- [@iotandwearablesguy](https://github.com/iotandwearablesguy) - Idea & Initial work

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

- [Thank You SimpleMaps.com for the csv file of city data](https://simplemaps.com/data/us-cities)



 ## LICENSE

_O'Shea-Dishongh No Harm License© 2022 Terrance O'Shea & Katherine Dishongh_

<http://github.com/terryo-iotandwearablesguy>

Subject to the following conditions:

 1. That the below copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

 2. That any use of the Software under this license never be used for harmful purposes, permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction,including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to
 do so subject to the aforementioned restrictions.

"Harmful purposes" as used herein means to not harm destroy or otherwise restrict humans, animals, or the environment. "Harmful purposes" includes, inter alia, the creation of weaponry, the guidance of weapons, limitation of food and services, damaging, contaminating or otherwise hazardous to the environmnet in any form, and/or restricting the freedom or movement of people.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE

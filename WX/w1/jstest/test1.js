#!/usr/bin/env bun

/* 
This is for testing shell js scripts
*/

// --- Imports ---
import boxen from "boxen"
import chalk from "chalk"

// --- Variables ---

// --- Main Function Call ---

function main() {
  boxen1()
}

// -- Sub Function Calls

// Box Function
function boxen1() {
  console.log(
    boxen("unicorns love rainbows", {
      title: "magical",
      titleAlignment: "center",
      bordercolor: "green",
    })
  )
}

// Printing text function
function writeText(params) {
  console.log(chalk.blue("Hello world!"))
}

// --- execution zone ---
main()

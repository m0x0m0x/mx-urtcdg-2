#!/usr/bin/env bun

/* 
This is for testing shell js scripts
*/

import boxen from "boxen"
import chalk from "chalk"

// --- Main Function Call ---

function main() {
  writeText()
}

// -- Sub Function Calls

// Box Function
function boxen1() {
  console.log(
    boxen("unicorns love rainbows", {
      title: "magical",
      titleAlignment: "center",
    })
  )
}

// Printing text function
function writeText(params) {
  console.log(chalk.blue("Hello world!"))
}

// --- execution zone ---
main()

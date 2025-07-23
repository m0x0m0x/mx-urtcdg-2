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
  const t1 = "Box Test 1"
  const tit = "Titlez"

  console.log(
    boxen(chalk.yellow(t1), {
      padding: 0.7,
      title: tit,
      titleAlignment: "center",
      borderColor: "green",
      borderStyle: "doubleSingle",
    })
  )
}

// Printing text function
function writeText(params) {
  console.log(chalk.blue("Hello world!"))
}

// --- execution zone ---
main()

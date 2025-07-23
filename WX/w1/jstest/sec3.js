#!/usr/bin/env bun

/* 
Section3 Chapter 20 Work
*/

// --- Imports ---
import boxen from "boxen"
import chalk from "chalk"

// --- Main Function ---

function title(textz, titlez) {
  console.log(
    boxen(chalk.yellow(textz), {
      padding: 0.7,
      textAlign: "center",
      title: titlez,
      titleAlignment: "center",
      borderColor: "green",
      borderStyle: "doubleSingle",
    })
  )
}

// -- Sub Function called in Main ---

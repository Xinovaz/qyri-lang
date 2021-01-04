#!/usr/bin/env node

// Wax is in development. Most of this is testing

require('isomorphic-fetch');
var program = require('commander');
const yaml = require('js-yaml');
var fs = require('fs');


// Dropbox access
var Dropbox = require('dropbox').Dropbox;
var ACCESS_TOKEN = 'sl.AmcnwgY6x6cEX38_MnFyxJ7zmURHdH-7UyuepViq896Zf1AsBksFjBSFCCCduvwXcezexA1SfnAQqsvr1wIIgvBDjsVCxo-g-hrcnUCNB_jNUoCVnnUvNHY-FM09SD0fjeO_0KY';
var dbx = new Dropbox({ accessToken: ACCESS_TOKEN });


program // Basic Info
	.version('0.0.1')
	.name("wax")
	.usage("[options] command [options] [arguments]");

program // Default options
	.option('-d, --debug', 'run Wax in debug mode', false);

// Wax Commands

program // wax new
	.command('new <name> [authors...]')
	.option('-l, --lib', 'create as library', false)
	.option('-p, --prod', 'create production environment for shipping', false)
	.description('create a new Qyri project')
	.action((name, authors, cmd) => {
		// Create an entire project
		var authorstr = authors.join('\n    - ');

		if (authors.length == 0) {
			var authors = ['lazy']
		}

		fs.mkdirSync(`./${name}`) // Create project parent folder
		fs.writeFileSync(`./${name}/index.yml`, // Create package data file

`---
metadata:
  name: ${name}
  authors:
    - ${authorstr}
  version: a0.0.1

dependencies:
  - `
		);
		fs.writeFileSync(`./${name}/anchor.yml`,
`---
- ${name}
- '00'
- |-
  ${require('os').userInfo().username}
  as
  ${authors[0]}
...
---
# Replace 'TRUE' with 'FALSE' to activate your package.
ignore: TRUE`
		);


		if (!cmd.prod) {
			fs.mkdirSync(`./${name}/src`); // Create source code folder
			if (!cmd.lib) fs.writeFileSync(`./${name}/src/main.qi`, // Create default source code
`fn main = function(out Void) {
	print("Hello, world!");
}
`
			);

			fs.mkdirSync(`./${name}/bin`); // Create binary container
			fs.mkdirSync(`./${name}/bin/debug`); // Create debugging folder
			if (cmd.lib) {
				fs.mkdirSync(`./${name}/bin/tests`); // Create tests directory if library
				fs.writeFileSync(`./${name}/src/compile.qi`, ''); // Create compilation file
			}
		} else {
			fs.mkdirSync(`./${name}/meta`); // Create production metadata folder for licenses etc.
		}



	});

program // wax verify
	.command('verify')
	.option('-n, --encoding', 'search for a YAML anchor with an encoding', 'utf8')
	.description('verify a directory as a Qyri project')
	.action((cmd) => {
		try {
			// get the anchor
			const anchor = yaml.safeLoadAll(fs.readFileSync(process.cwd() + "/anchor.yml", cmd.encoding));
			// verify the anchor
			if (anchor[0][1] == '00') {
				if (anchor[1]['ignore']) {
					console.log('This project is valid, but the ignore flag is set.');
				} else {
					console.log('This project is valid.')
				}
			}
		} catch (e) {
			if (e.code == 'ENOENT') {
				console.log("Wax could not find anchor.yml in this directory. Try changing anchor.yaml to anchor.yml.");
				process.exit();
			} else {
				console.log(e);
			}
		}
	});



// Use from command line
program.parse(process.argv);
//
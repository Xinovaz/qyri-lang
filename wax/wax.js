#!/usr/bin/env node

// Wax is in development. Most of this is testing

require('isomorphic-fetch');
var program = require('commander');
var fs = require('fs');


program // Basic Info
	.version('0.0.1')
	.name("wax")
	.usage("command [options] [arguments]");

program // Options
	.option('-q, --quiet', 'run Wax in quiet mode');

// Wax Commands

program // wax new
	.command('new <name>')
	.option('-l, --lib', 'create as library')
	.option('-p, --prod', 'create production environment for shipping')
	.description('create a new Qyri project')
	.action((name) => {
		// Create an entire project
		console.log(`Command in development. Name is ${name}`);
	});

// Use from command line
program.parse(process.argv);
//


// Dropbox access
var Dropbox = require('dropbox').Dropbox;
var ACCESS_TOKEN = 'sl.AmcnwgY6x6cEX38_MnFyxJ7zmURHdH-7UyuepViq896Zf1AsBksFjBSFCCCduvwXcezexA1SfnAQqsvr1wIIgvBDjsVCxo-g-hrcnUCNB_jNUoCVnnUvNHY-FM09SD0fjeO_0KY';
var dbx = new Dropbox({ accessToken: ACCESS_TOKEN });
bits32 = ['arm', 'ia32', 'x32', 'ppc', 's390']
bits64 = ['arm64', 'ppc64', 'x64', 'mips', 'mipsel', 's390x']
if (bits32.includes(process.arch)) {
	_arch = 32;
} else if (bits64.includes(process.arch)) {
	_arch = 64;
} else {
	_arch = 0;
}

function _integer(number, size=_arch) {
	this.size = size;
	this.canonical = number;
}

function _float(number, size=_arch) {
	this.size = size;
	this.canonical = number;
}

function _string(source_str) {
	this.canonical = source_str;
}

function _boolean(source_bool) {
	this.canonical = source_bool;
}

function _array(source_array) {
	this.canonical = source_array;
}

module.exports = {
	_integer: _integer,
	_float: _float,
	_string: _string,
	_boolean: _boolean,
	_array: _array
};
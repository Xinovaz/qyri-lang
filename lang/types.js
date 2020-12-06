function _integer(number_as_str) {
	this.canonical = parseInt(number_as_str, 10);
}

module.exports = {
	_integer: _integer
};
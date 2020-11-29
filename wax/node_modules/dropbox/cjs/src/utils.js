"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.getBaseURL = getBaseURL;
exports.httpHeaderSafeJson = httpHeaderSafeJson;
exports.getTokenExpiresAtDate = getTokenExpiresAtDate;
exports.isWindowOrWorker = isWindowOrWorker;

function getSafeUnicode(c) {
  var unicode = "000".concat(c.charCodeAt(0).toString(16)).slice(-4);
  return "\\u".concat(unicode);
}

function getBaseURL(host) {
  return "https://".concat(host, ".dropboxapi.com/2/");
} // source https://www.dropboxforum.com/t5/API-support/HTTP-header-quot-Dropbox-API-Arg-quot-could-not-decode-input-as/m-p/173823/highlight/true#M6786


function httpHeaderSafeJson(args) {
  return JSON.stringify(args).replace(/[\u007f-\uffff]/g, getSafeUnicode);
}

function getTokenExpiresAtDate(expiresIn) {
  return new Date(Date.now() + expiresIn * 1000);
}
/* global WorkerGlobalScope */


function isWindowOrWorker() {
  return typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope // eslint-disable-line no-restricted-globals
  || typeof module === 'undefined' || typeof window !== 'undefined';
}
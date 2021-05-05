<?php

require_once __DIR__ . '/_common.php';

assert_eq(integrate_arguments_null(null), null);
assert_throw(function () { integrate_arguments_null(); }, "ArgumentCountError", 0, "integrate_arguments_null(): expects at least 1 parameter(s), 0 given");
assert_throw(function () { integrate_arguments_null(1); }, "TypeError", 0, "type error: must be of type null, int given");

assert_eq(integrate_arguments_long(1, 2), 3);
assert_eq(integrate_arguments_long(1, "2"), 3);
assert_throw(function () { integrate_arguments_long("1", "2"); }, "TypeError", 0, "type error: must be of type int, string given");

assert_eq(integrate_arguments_double(1.0), 1.0);
assert_throw(function () { integrate_arguments_double(1); }, "TypeError", 0, "type error: must be of type float, int given");

assert_eq(integrate_arguments_string("hello", "world"), "hello, world");
assert_eq(integrate_arguments_string("hello", 123), "hello, 123");
assert_throw(function () { integrate_arguments_string(1, 2); }, "TypeError", 0, "type error: must be of type string, int given");

assert_eq(integrate_arguments_array(["a" => 1]), ["a" => 1, "foo" => "bar"]);
assert_throw(function () { integrate_arguments_array(null); }, "TypeError", 0, "type error: must be of type array, null given");

$obj = new stdClass();
$obj->a = 1;
assert_object(integrate_arguments_object($obj), "stdClass", ["a" => 1, "foo" => "bar"]);
assert_throw(function () { integrate_arguments_object(1); }, "TypeError", 0, "type error: must be of type object, int given");

assert_throw(function () { integrate_arguments_optional(); }, "ArgumentCountError", 0, "integrate_arguments_optional(): expects at least 1 parameter(s), 0 given");
assert_eq(integrate_arguments_optional("foo"), "foo: false");
assert_eq(integrate_arguments_optional("foo", true), "foo: true");
assert_eq(integrate_arguments_optional("foo", true, "bar"), "foo: true");

// subclassing is not transitive when you can remove required parameters and add optional parameters
class C {
    foo(x, y) {
    }
}
class D extends C {
    foo(x) {
    }
}
class E extends D {
    foo(x, y) {
    }
}
var c;
var d;
var e;
c = e;
var r = c.foo(1, 1);
var r2 = e.foo(1, '');
function _classCallCheck(instance, Constructor) {
    if (!(instance instanceof Constructor)) throw new TypeError("Cannot call a class as a function");
}
function _getPrototypeOf(o) {
    return _getPrototypeOf = Object.setPrototypeOf ? Object.getPrototypeOf : function _getPrototypeOf(o) {
        return o.__proto__ || Object.getPrototypeOf(o);
    }, _getPrototypeOf(o);
}
function _inherits(subClass, superClass) {
    if ("function" != typeof superClass && null !== superClass) throw new TypeError("Super expression must either be null or a function");
    subClass.prototype = Object.create(superClass && superClass.prototype, {
        constructor: {
            value: subClass,
            writable: !0,
            configurable: !0
        }
    }), superClass && _setPrototypeOf(subClass, superClass);
}
function _possibleConstructorReturn(self, call) {
    return call && ("object" === _typeof(call) || "function" == typeof call) ? call : (function(self) {
        if (void 0 === self) throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
        return self;
    })(self);
}
function _setPrototypeOf(o, p) {
    return _setPrototypeOf = Object.setPrototypeOf || function _setPrototypeOf(o, p) {
        return o.__proto__ = p, o;
    }, _setPrototypeOf(o, p);
}
var _typeof = function(obj) {
    return obj && "undefined" != typeof Symbol && obj.constructor === Symbol ? "symbol" : typeof obj;
}, Base1 = function() {
    "use strict";
    _classCallCheck(this, Base1);
}, Derived = function(Base) {
    "use strict";
    function Derived() {
        return _classCallCheck(this, Derived), _possibleConstructorReturn(this, _getPrototypeOf(Derived).apply(this, arguments));
    }
    return _inherits(Derived, Base), Derived;
}(Base1), Derived2 = function(Derived) {
    "use strict";
    function Derived2() {
        return _classCallCheck(this, Derived2), _possibleConstructorReturn(this, _getPrototypeOf(Derived2).apply(this, arguments));
    }
    return _inherits(Derived2, Derived), Derived2;
}(Derived), OtherDerived = function(Base) {
    "use strict";
    function OtherDerived() {
        return _classCallCheck(this, OtherDerived), _possibleConstructorReturn(this, _getPrototypeOf(OtherDerived).apply(this, arguments));
    }
    return _inherits(OtherDerived, Base), OtherDerived;
}(Base1);
foo1(function(x) {
    return null;
}), foo2(function(x) {
    return [
        ""
    ];
}), foo3(function(x) {
    return null;
}), foo4(function(x, y) {
    return "";
}), foo5(function(x) {
    return null;
}), foo6(function(x) {
    return null;
}), foo11(function(x, y) {
    return null;
}), foo15(function(x) {
    return null;
}), foo16(function(x) {
    return null;
}), foo17(function(x) {
    return null;
}), foo18(function(x) {
    return null;
});
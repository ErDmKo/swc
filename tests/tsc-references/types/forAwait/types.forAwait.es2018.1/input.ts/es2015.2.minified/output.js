function AsyncGenerator(gen) {
    var front, back;
    function resume(key, arg) {
        try {
            var result = gen[key](arg), value = result.value, wrappedAwait = value instanceof _AwaitValue;
            Promise.resolve(wrappedAwait ? value.wrapped : value).then(function(arg) {
                if (wrappedAwait) {
                    resume("next", arg);
                    return;
                }
                settle(result.done ? "return" : "normal", arg);
            }, function(err) {
                resume("throw", err);
            });
        } catch (err) {
            settle("throw", err);
        }
    }
    function settle(type, value) {
        switch(type){
            case "return":
                front.resolve({
                    value: value,
                    done: !0
                });
                break;
            case "throw":
                front.reject(value);
                break;
            default:
                front.resolve({
                    value: value,
                    done: !1
                });
                break;
        }
        (front = front.next) ? resume(front.key, front.arg) : back = null;
    }
    this._invoke = function(key, arg) {
        return new Promise(function(resolve, reject) {
            var request = {
                key: key,
                arg: arg,
                resolve: resolve,
                reject: reject,
                next: null
            };
            back ? back = back.next = request : (front = back = request, resume(key, arg));
        });
    }, "function" != typeof gen.return && (this.return = void 0);
}
function _asyncIterator(iterable) {
    var method;
    if ("function" == typeof Symbol) {
        if (Symbol.asyncIterator && null != (method = iterable[Symbol.asyncIterator])) return method.call(iterable);
        if (Symbol.iterator && null != (method = iterable[Symbol.iterator])) return method.call(iterable);
    }
    throw new TypeError("Object is not async iterable");
}
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg), value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    info.done ? resolve(value) : Promise.resolve(value).then(_next, _throw);
}
function _AwaitValue(value) {
    this.wrapped = value;
}
function _f1() {
    return (_f1 = (function(fn) {
        return function() {
            var self = this, args = arguments;
            return new Promise(function(resolve, reject) {
                var gen = fn.apply(self, args);
                function _next(value) {
                    asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
                }
                function _throw(err) {
                    asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
                }
                _next(void 0);
            });
        };
    })(function*() {
        var _iteratorError, _iteratorAbruptCompletion = !1, _didIteratorError = !1;
        try {
            for(var _step, _iterator = _asyncIterator(asyncIterable); _iteratorAbruptCompletion = !(_step = yield _iterator.next()).done; _iteratorAbruptCompletion = !1);
        } catch (err) {
            _didIteratorError = !0, _iteratorError = err;
        } finally{
            try {
                _iteratorAbruptCompletion && null != _iterator.return && (yield _iteratorError.return());
            } finally{
                if (_didIteratorError) throw _iteratorError;
            }
        }
        var _iteratorError1, _iteratorAbruptCompletion1 = !1, _didIteratorError1 = !1;
        try {
            for(var _step1, _iterator1 = _asyncIterator(iterable); _iteratorAbruptCompletion1 = !(_step1 = yield _iterator1.next()).done; _iteratorAbruptCompletion1 = !1);
        } catch (err1) {
            _didIteratorError1 = !0, _iteratorError1 = err1;
        } finally{
            try {
                _iteratorAbruptCompletion1 && null != _iterator1.return && (yield _iteratorError1.return());
            } finally{
                if (_didIteratorError1) throw _iteratorError1;
            }
        }
        var _iteratorError4, _iteratorAbruptCompletion3 = !1, _didIteratorError3 = !1;
        try {
            for(var _step2, _iterator3 = _asyncIterator(iterableOfPromise); _iteratorAbruptCompletion3 = !(_step2 = yield _iterator3.next()).done; _iteratorAbruptCompletion3 = !1);
        } catch (err2) {
            _didIteratorError3 = !0, _iteratorError4 = err2;
        } finally{
            try {
                _iteratorAbruptCompletion3 && null != _iterator3.return && (yield _iteratorError4.return());
            } finally{
                if (_didIteratorError3) throw _iteratorError4;
            }
        }
        var _iteratorError7, _iteratorAbruptCompletion5 = !1, _didIteratorError5 = !1;
        try {
            for(var _step3, _iterator5 = _asyncIterator(asyncIterable); _iteratorAbruptCompletion5 = !(_step3 = yield _iterator5.next()).done; _iteratorAbruptCompletion5 = !1);
        } catch (err3) {
            _didIteratorError5 = !0, _iteratorError7 = err3;
        } finally{
            try {
                _iteratorAbruptCompletion5 && null != _iterator5.return && (yield _iteratorError7.return());
            } finally{
                if (_didIteratorError5) throw _iteratorError7;
            }
        }
        var _iteratorError10, _iteratorAbruptCompletion7 = !1, _didIteratorError7 = !1;
        try {
            for(var _step4, _iterator7 = _asyncIterator(iterable); _iteratorAbruptCompletion7 = !(_step4 = yield _iterator7.next()).done; _iteratorAbruptCompletion7 = !1);
        } catch (err4) {
            _didIteratorError7 = !0, _iteratorError10 = err4;
        } finally{
            try {
                _iteratorAbruptCompletion7 && null != _iterator7.return && (yield _iteratorError10.return());
            } finally{
                if (_didIteratorError7) throw _iteratorError10;
            }
        }
        var _iteratorError13, _iteratorAbruptCompletion9 = !1, _didIteratorError9 = !1;
        try {
            for(var _step5, _iterator9 = _asyncIterator(iterableOfPromise); _iteratorAbruptCompletion9 = !(_step5 = yield _iterator9.next()).done; _iteratorAbruptCompletion9 = !1);
        } catch (err5) {
            _didIteratorError9 = !0, _iteratorError13 = err5;
        } finally{
            try {
                _iteratorAbruptCompletion9 && null != _iterator9.return && (yield _iteratorError13.return());
            } finally{
                if (_didIteratorError9) throw _iteratorError13;
            }
        }
    })).apply(this, arguments);
}
function _f2() {
    return (_f2 = (function(fn) {
        return function() {
            return new AsyncGenerator(fn.apply(this, arguments));
        };
    })(function*() {
        var _iteratorError, _iteratorAbruptCompletion = !1, _didIteratorError = !1;
        try {
            for(var _step, _iterator = _asyncIterator(asyncIterable); _iteratorAbruptCompletion = !(_step = yield _iterator.next()).done; _iteratorAbruptCompletion = !1);
        } catch (err) {
            _didIteratorError = !0, _iteratorError = err;
        } finally{
            try {
                _iteratorAbruptCompletion && null != _iterator.return && (yield _iteratorError.return());
            } finally{
                if (_didIteratorError) throw _iteratorError;
            }
        }
        var _iteratorError2, _iteratorAbruptCompletion2 = !1, _didIteratorError1 = !1;
        try {
            for(var _step1, _iterator2 = _asyncIterator(iterable); _iteratorAbruptCompletion2 = !(_step1 = yield _iterator2.next()).done; _iteratorAbruptCompletion2 = !1);
        } catch (err1) {
            _didIteratorError1 = !0, _iteratorError2 = err1;
        } finally{
            try {
                _iteratorAbruptCompletion2 && null != _iterator2.return && (yield _iteratorError2.return());
            } finally{
                if (_didIteratorError1) throw _iteratorError2;
            }
        }
        var _iteratorError11, _iteratorAbruptCompletion6 = !1, _didIteratorError3 = !1;
        try {
            for(var _step2, _iterator6 = _asyncIterator(iterableOfPromise); _iteratorAbruptCompletion6 = !(_step2 = yield _iterator6.next()).done; _iteratorAbruptCompletion6 = !1);
        } catch (err2) {
            _didIteratorError3 = !0, _iteratorError11 = err2;
        } finally{
            try {
                _iteratorAbruptCompletion6 && null != _iterator6.return && (yield _iteratorError11.return());
            } finally{
                if (_didIteratorError3) throw _iteratorError11;
            }
        }
        var _iteratorError17, _iteratorAbruptCompletion10 = !1, _didIteratorError5 = !1;
        try {
            for(var _step3, _iterator10 = _asyncIterator(asyncIterable); _iteratorAbruptCompletion10 = !(_step3 = yield _iterator10.next()).done; _iteratorAbruptCompletion10 = !1);
        } catch (err3) {
            _didIteratorError5 = !0, _iteratorError17 = err3;
        } finally{
            try {
                _iteratorAbruptCompletion10 && null != _iterator10.return && (yield _iteratorError17.return());
            } finally{
                if (_didIteratorError5) throw _iteratorError17;
            }
        }
        var _iteratorError20, _iteratorAbruptCompletion17 = !1, _didIteratorError7 = !1;
        try {
            for(var _step4, _iterator17 = _asyncIterator(iterable); _iteratorAbruptCompletion17 = !(_step4 = yield _iterator17.next()).done; _iteratorAbruptCompletion17 = !1);
        } catch (err4) {
            _didIteratorError7 = !0, _iteratorError20 = err4;
        } finally{
            try {
                _iteratorAbruptCompletion17 && null != _iterator17.return && (yield _iteratorError20.return());
            } finally{
                if (_didIteratorError7) throw _iteratorError20;
            }
        }
        var _iteratorError23, _iteratorAbruptCompletion19 = !1, _didIteratorError9 = !1;
        try {
            for(var _step5, _iterator19 = _asyncIterator(iterableOfPromise); _iteratorAbruptCompletion19 = !(_step5 = yield _iterator19.next()).done; _iteratorAbruptCompletion19 = !1);
        } catch (err5) {
            _didIteratorError9 = !0, _iteratorError23 = err5;
        } finally{
            try {
                _iteratorAbruptCompletion19 && null != _iterator19.return && (yield _iteratorError23.return());
            } finally{
                if (_didIteratorError9) throw _iteratorError23;
            }
        }
    })).apply(this, arguments);
}
"function" == typeof Symbol && Symbol.asyncIterator && (AsyncGenerator.prototype[Symbol.asyncIterator] = function() {
    return this;
}), AsyncGenerator.prototype.next = function(arg) {
    return this._invoke("next", arg);
}, AsyncGenerator.prototype.throw = function(arg) {
    return this._invoke("throw", arg);
}, AsyncGenerator.prototype.return = function(arg) {
    return this._invoke("return", arg);
};
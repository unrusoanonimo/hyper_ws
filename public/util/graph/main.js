var Rang = /** @class */ (function () {
    function Rang(start, end) {
        this.start = start;
        this.end = end;
    }
    Rang.prototype.map = function (fn) {
        return new Rang(fn(this.start), fn(this.end));
    };
    Rang.prototype.size = function () {
        if (typeof this.start !== "number" && typeof this.start !== "bigint")
            throw new TypeError("size coan only be used on Range<number|bigint>");
        return (this.end - this.start);
    };
    return Rang;
}());
Array.prototype.mut_map = function (fn) {
    for (var i = 0; i < this.length; i++) {
        this[i] = fn(this[i], i);
    }
    return this;
};
Array.prototype.bounds = function (fn, auto_map) {
    if (this.length <= 0)
        throw new Error("Array needs atleast one element to get it's bounds");
    fn !== null && fn !== void 0 ? fn : (fn = function (v) { return v; });
    var start_i = 0;
    var end_i = 0;
    var start = fn(this[0]);
    var end = fn(this[0]);
    for (var i = 1; i < this.length; i++) {
        var item = fn(this[i]);
        if (item < start) {
            start = item;
            start_i = i;
        }
        else if (item > end) {
            end = item;
            end_i = i;
        }
    }
    if (auto_map)
        return new Rang(start, end);
    return new Rang(this[start_i], this[end_i]);
};
var data = [
    [1, 8],
    [4, 7],
    [2, 5],
    [6, 1],
    [0, 4]
];
var CANV_W = 1200;
var CANV_H = 800;
var canv = document.querySelector("canvas");
document.addEventListener("DOMContentLoaded", function () {
    canv = document.querySelector("canvas");
    canv.width = CANV_W;
    canv.height = CANV_H;
    render_data(data);
    document.querySelector("#generate").addEventListener("click", function () {
        console.log((document.querySelector("#data").value));
        render_data(JSON.parse(document.querySelector("#data").value));
    });
});
function render_function(f, precision) {
    if (precision === void 0) { precision = 1000; }
    var data = Array(precision);
    for (var i = 0; i < precision; i++)
        data[i] = [i, f(i, precision)];
    render_data(data);
}
function render_data(data, canvas) {
    if (canvas === void 0) { canvas = canv; }
    data.sort(function (a, b) { return a[0] - b[0]; });
    var Y_RANGE = data.bounds(function (v) { return v[1]; }, true);
    var X_RANGE = new Rang(data[0][0], data[data.length - 1][0]);
    {
        var x_1 = CANV_W / X_RANGE.size();
        var y_1 = CANV_H / Y_RANGE.size();
        data.mut_map(function (v) { return [(v[0] - X_RANGE.start) * x_1, (v[1] - Y_RANGE.start) * y_1]; });
    }
    var ctx = canv.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.beginPath();
    var _a = data[0], x = _a[0], y = _a[1];
    ctx.moveTo(x, CANV_H - y);
    for (var i = 1; i < data.length; i++) {
        var _b = data[i], x_2 = _b[0], y_2 = _b[1];
        ctx.lineTo(x_2, CANV_H - y_2);
    }
    for (var i = 1; i < 10; i++) {
        var x_3 = canvas.width / 10 * i;
        ctx.moveTo(x_3, CANV_H);
        ctx.lineTo(x_3, CANV_H - 10);
    }
    ctx.stroke();
}

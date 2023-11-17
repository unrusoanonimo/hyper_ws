type Data = [number, number][];
type N = number | bigint;
class Rang<T> {
    constructor(
        public readonly start: T,
        public readonly end: T,
    ) { }
    map<S>(fn: (v: T) => S): Rang<S> {
        return new Rang(fn(this.start), fn(this.end));
    }
    size(): N & T {
        if (typeof this.start !== "number" && typeof this.start !== "bigint")
            throw new TypeError("size coan only be used on Range<number|bigint>");
        return (this.end as any - (this.start as unknown as any)) as N & T
    }

}
interface Array<T> {
    mut_map(fn: (v: T, i: number) => T): this;
    bounds(): Rang<T>;
    bounds<S>(fn: (v: T) => S, auto_map?: false): Rang<T>;
    bounds<S>(fn: (v: T) => S, auto_map: true): Rang<S>;
}
Array.prototype.mut_map = function <T>(this: Array<T>, fn: (v: T, i: number) => T): typeof this {
    for (let i = 0; i < this.length; i++) {
        this[i] = fn(this[i], i);
    }
    return this;
}
Array.prototype.bounds = function <T, S>(this: Array<T>, fn?: (v: T) => S, auto_map?: boolean): Rang<T | S> {
    if (this.length <= 0)
        throw new Error("Array needs atleast one element to get it's bounds");
    fn ??= v => v as any;

    let start_i = 0;
    let end_i = 0;
    let start: S = fn(this[0]);
    let end: S = fn(this[0]);
    for (let i = 1; i < this.length; i++) {
        const item = fn(this[i]);

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
}

const data: Data = [
    [1, 8],
    [4, 7],
    [2, 5],
    [6, 1],
    [0, 4]
]



const CANV_W = 1200;
const CANV_H = 800;

let canv = document.querySelector("canvas")!;
document.addEventListener("DOMContentLoaded", () => {
    canv = document.querySelector("canvas")!;
    canv.width = CANV_W;
    canv.height = CANV_H;

    render_data(data);
    document.querySelector<HTMLButtonElement>("#generate")!.addEventListener("click", () => {
        console.log((document.querySelector<HTMLTextAreaElement>("#data")!.value));

        render_data(JSON.parse(document.querySelector<HTMLTextAreaElement>("#data")!.value));
    });
})

function render_function(f: (x: number, total) => number, precision = 1000) {
    const data: Data = Array(precision);
    for (let i = 0; i < precision; i++)
        data[i] = [i, f(i, precision)];

    render_data(data);

}

function render_data(data: Data, canvas: HTMLCanvasElement = canv) {
    data.sort((a, b) => a[0] - b[0]);
    const Y_RANGE = data.bounds(v => v[1], true);
    const X_RANGE = new Rang(data[0][0], data[data.length - 1][0]);
    {
        const x = CANV_W / X_RANGE.size();
        const y = CANV_H / Y_RANGE.size();
        data.mut_map(v => [(v[0] - X_RANGE.start) * x, (v[1] - Y_RANGE.start) * y])
    }

    const ctx = canv.getContext("2d")!;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.beginPath();
    const [x, y] = data[0];
    ctx.moveTo(x, CANV_H - y);
    for (let i = 1; i < data.length; i++) {
        const [x, y] = data[i];
        ctx.lineTo(x, CANV_H - y)

    }

    for (let i = 1; i < 10; i++) {
        const x = canvas.width / 10 * i
        ctx.moveTo(x, CANV_H);
        ctx.lineTo(x, CANV_H - 10);
    }
    ctx.stroke()
}

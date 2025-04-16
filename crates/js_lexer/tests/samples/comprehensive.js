// Single-line comment
/* Multi-line
   comment */

// Variable declarations with different types
let stringVar = 'single quotes';
const templateStr = `template ${stringVar}`;
var numberVar = 42;
const floatVar = 3.14159;
let hexNumber = 0xFF;
const octalNum = 0o777;
const binNum = 0b1010;
const bigIntNum = 9007199254740991n;

// Array and object literals
const arr = [1, 'two', { three: 3 }];
const obj = {
    prop1: 'value1',
    'quoted-prop': 2,
    [`computed${stringVar}`]: true,
    method() { return this; }
};

// Function declarations and expressions
function normalFunc(param1, param2 = 'default') {
    return param1 + param2;
}

const arrowFunc = (x, y) => x * y;
const asyncFunc = async () => {
    await Promise.resolve();
};

// Classes and inheritance
class BaseClass {
    #privateField = 'private';
    static staticProp = 'static';
    
    constructor() {
        this.publicField = 'public';
    }

    get getter() { return this.publicField; }
    set setter(value) { this.publicField = value; }
}

class ChildClass extends BaseClass {
    constructor() {
        super();
    }
}

// Control structures and loops
if (true) {
    console.log('true');
} else if (false) {
    console.log('false');
} else {
    console.log('else');
}

for (let i = 0; i < 5; i++) {
    continue;
}

for (const key in obj) {
    if (obj.hasOwnProperty(key)) break;
}

for (const item of arr) {
    console.log(item);
}

while (false) {
    break;
}

do {
    console.log('once');
} while (false);

switch (numberVar) {
    case 1:
        break;
    default:
        console.log('default');
}

// Try-catch and error handling
try {
    throw new Error('test');
} catch (e) {
    console.error(e);
} finally {
    console.log('cleanup');
}

// Regular expressions
const regex = /^hello[world]+$/gi;
const regexObj = new RegExp('pattern', 'u');

// Modules
export const exportedVar = 'exported';
export default class DefaultExport {}
import { something } from './module';

// Modern features
const { destructured, ...rest } = obj;
const [...spread] = arr;
const nullCoalesce = null ?? 'default';
const optionalChain = obj?.prop?.method?.();

// Numeric separators
const bigNumber = 1_000_000;

// Symbols and generators
const symbol = Symbol('description');
function* generator() {
    yield 1;
    yield* [2, 3, 4];
}

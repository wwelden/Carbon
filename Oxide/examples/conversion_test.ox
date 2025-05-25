// Test conversion functions

let num = 255;
print("Decimal: " + toString(num));
print("Binary: " + toBinary(num));
print("Hex: " + toHex(num));

let binaryStr = "11111111";
print("Binary '" + binaryStr + "' to decimal: " + toString(toDecimal(binaryStr)));

let hexStr = "FF";
print("Hex '" + hexStr + "' to decimal: " + toString(toDecimal(hexStr)));

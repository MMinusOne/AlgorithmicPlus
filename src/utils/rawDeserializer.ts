const textDecoder = new TextDecoder("utf-8");

export function rawJSONDeserializer(arrayBuffer: ArrayBuffer) { 
    const typedArray = new Uint8Array(arrayBuffer);
    return JSON.parse(textDecoder.decode(typedArray));
}
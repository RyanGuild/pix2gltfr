import {image_to_gltf} from "./pix2gltfr/Cargo.toml";
import { partial, map, flow, curryRight, curry } from "lodash-es"
import { ArrayIterator, identity, partialRight } from "lodash";

console.log(image_to_gltf);

const fileInput = document.querySelector("#file-input") as HTMLInputElement;


const fileToDataURL = async (file: File): Promise<string | ArrayBuffer> => {
  const tempFR = new FileReader();
  return new Promise((resolve, reject) => {
    tempFR.onerror = () => {
      tempFR.abort();
      reject(new DOMException("Problem parsing input file."));
    };

    tempFR.onload = () => {
      resolve(tempFR.result);
    };

    tempFR.readAsDataURL(file);
  });  
}
const fileEventToFileList = (event: Event & { target: HTMLInputElement }): FileList => (event.target.files)

const dataUrlToImage = (dataUrl: string): HTMLImageElement => {
  const img = new Image();
  img.src = dataUrl;
  return img;
}

const imageToCanvas = (image: HTMLImageElement): HTMLCanvasElement => {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  canvas.width = image.width;
  canvas.height = image.height;
  ctx.drawImage(image, 0, 0);
  return canvas;
}



const canvasToImageData = (canvas: HTMLCanvasElement):ImageData => canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height);


const mapFn = partialRight(map);


const fileEventToDataArray = flow(
  fileEventToFileList,
  Array.from,
  curryRight(map)(fileToDataURL),
  Promise.all

);





addEventListener("change", fileInputHandler as EventListener);

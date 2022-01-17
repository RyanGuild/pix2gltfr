import { useState, useEffect, useCallback, useRef, Suspense } from "react";
import * as React from "react";
import { Canvas, ObjectMap, useLoader, } from "@react-three/fiber";
import { GLTF, GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader";
import { image_to_gltf } from "./pix2gltfr/Cargo.toml";

console.log(image_to_gltf);


export const RenderGLTF: React.FC<{ gltf: GLTF }> = ({ gltf }) => (<primitive object={gltf.scene} position={[0, 0, 0]} />)

export const ImageProcessor: React.FC<{ dataurl: string }> = ({ dataurl }) => {
  const imgRef = useRef<HTMLImageElement>();
  const inCanvasRef = useRef<HTMLCanvasElement>();

  const [gltf_src, setGltfSrc] = useState<ReturnType<typeof image_to_gltf>>();
  const [gltf, setGltf] = useState<GLTF>();
  const gltfLoader = React.useMemo(() => new GLTFLoader(), []);

  useEffect(() => {
    if (gltf_src) {
      gltfLoader.parse(gltf_src, "default", gltf => setGltf(gltf));
    }
  }, [gltf_src]);


  useEffect(() => {
    if (gltf) {
      console.log(gltf);
    }
  },[gltf]);

  return (
    <div>
      <img onLoad={() => {
        const img = imgRef.current;
        const inCanvas = inCanvasRef.current;
        inCanvas.width = img.width;
        inCanvas.height = img.height;
        const ctx = inCanvas.getContext("2d");
        ctx.drawImage(img, 0, 0);
        const data = ctx.getImageData(0, 0, img.width, img.height);
        const gltf = image_to_gltf(data);
        console.log(JSON.parse(gltf))
        setGltfSrc(gltf);
      }} ref={imgRef} src={dataurl} />
      <canvas ref={inCanvasRef} />
      <Canvas 
        style={ { width: "100%", height: "1000px", backgroundColor: "gray" } }
      camera={{
        position: [0, 0, 10],
      }}>
          <ambientLight />
          <pointLight position={[10, 10, 10]} />
          <mesh>
            <meshStandardMaterial attach="material" color="white" />
          </mesh>
          {gltf && <RenderGLTF gltf={gltf} />}
      </Canvas>
    </div>
  )
}


export const App: React.FC = () => {
  const [files, setFiles] = useState<FileList>(null);
  const [dataUris, setDataUris] = useState<string[]>([]);
  const onChangeInput = useCallback((event: React.ChangeEvent<HTMLInputElement>) => event.target.files && setFiles(event.target.files), [setFiles]);

  useEffect(() => {
    const urls = Array.from<File>(files || []).map(URL.createObjectURL);
    setDataUris(urls);

    return () => urls.forEach(URL.revokeObjectURL);
  }, [files]);

  return (
    <div>
      <form>
        <label htmlFor="formFile">Choose images to convert to gltf</label>
        <input onChange={onChangeInput} name="formFile" type="file" id="file-input" />
      </form>
      {dataUris.map((dataUri, i) =>
        (<ImageProcessor key={i} dataurl={dataUri} />))}
    </div>
  )
}
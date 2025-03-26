import { invoke } from "@tauri-apps/api/core";

export class ParsedImage {
    blobUrl: string;
    mimeType: string;

    constructor(blobUrl: string, mimeType: string) {
        this.blobUrl = blobUrl;
        this.mimeType = mimeType;
    }

    static parseImageType(filePath: string): string {
        const fileExtension = filePath.split(".").pop();
        switch (fileExtension) {
            case "jpg":
            case "jpeg":
                return "image/jpeg";
            case "png":
                return "image/png";
            case "gif":
    
                return "image/gif";
            default:
                throw new Error(`Unsupported file type: ${fileExtension}`);
        }
    }

    static async fromFileList(fileList: string[]) {
        return fileList.map(async (fileName: string) => {
            const buf: number[] = await invoke("read_file", { fileName: fileName });
            const mimeType = this.parseImageType(fileName);
            const imageUrl = URL.createObjectURL(
                new Blob(
                    [new Uint8Array(buf)], 
                    { type: mimeType }
                )
            );

            return new ParsedImage(imageUrl, mimeType);
        })
    }

    static async fromDirFileList(directory: string, fileList: string[]) {
        return fileList.map(async (fileName: string) => {
            const buf: number[] = await invoke("read_directory_file", { dir: directory, fileName: fileName });
            const mimeType = this.parseImageType(fileName);
            const imageUrl = URL.createObjectURL(
                new Blob(
                    [new Uint8Array(buf)], 
                    { type: mimeType }
                )
            );

            return new ParsedImage(imageUrl, mimeType);
        })
    }
}
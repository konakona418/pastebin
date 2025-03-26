import { invoke } from "@tauri-apps/api/core";
import { ParsedImage } from "../image-container/parsed-image";

export class ParsedFolder {
    name: string;
    thumbNail: string;
    thumbMime: string;

    constructor(name: string, thumbNail: string, thumbMime: string) {
        this.name = name;
        this.thumbNail = thumbNail;
        this.thumbMime = thumbMime;
    }

    static async fromFolderList(folderList: string[]) {
        return folderList.map(async (folderName: string) => {
            const thumbName: string[] = await invoke("get_directory_files_page", { dir: folderName, page: 0, pageSize: 8 });
            console.log(thumbName);
            const buf: number[] = await invoke("read_directory_file", { dir: folderName, fileName: thumbName[0] });
            const thumbMime = ParsedImage.parseImageType(thumbName[0]);
            const thumbNail = URL.createObjectURL(
                new Blob(
                    [new Uint8Array(buf)], 
                    { type: thumbMime }
                )
            )
            return new ParsedFolder(folderName, thumbNail, thumbMime);
        })
    }
}
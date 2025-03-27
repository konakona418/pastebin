import { invoke } from "@tauri-apps/api/core";

export class Config {
    loadImageCnt: number;

    constructor() {
        this.loadImageCnt = 0;
    }
}

export const config = new Config();

export async function loadConfig() {
    const configData = JSON.parse(await invoke("read_config"));
    Object.assign(config, configData);
    console.log(config);
}

export async function saveConfig() {
    await invoke("write_config", { config: JSON.stringify(config) });
}
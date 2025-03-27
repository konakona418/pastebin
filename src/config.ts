import { invoke } from "@tauri-apps/api/core";

export class Config {
    loadImageCnt: number;

    constructor() {
        this.loadImageCnt = 0;
    }
}

let configLoaded = false;
export let config = new Config();
export const getConfig = async () => {
    if (!configLoaded) {
        await loadConfig();
    }
    return config;
}

export async function loadConfig() {
    const configData = JSON.parse(await invoke("read_config"));
    Object.assign(config, configData);
    console.log(config);
}

export async function saveConfig() {
    await invoke("write_config", { config: JSON.stringify(config) });
}
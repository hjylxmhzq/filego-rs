import { makeAutoObservable } from 'mobx';

export class Setting {
  download = {
    aria2Enabled: false,
    aria2RpcUrl: "",
  }
  constructor() {
    this.deserialize();
    makeAutoObservable(this);
  }
  enableAria2() {
    this.download.aria2Enabled = true;
    this.serialize();
  }
  disableAria2() {
    this.download.aria2Enabled = false;
    this.serialize();
  }
  setAria2RPCUrl(url: string) {
    this.download.aria2RpcUrl = url;
    this.serialize();
  }
  private serialize() {
    serialize('store.setting', this);
  }
  private deserialize() {
    let cached = deserialize('store.setting', this);
    if (cached) override(cached, this);
  }
}

export const setting = new Setting();

function serialize(key: string, s: any) {
  const str = JSON.stringify(s);
  localStorage.setItem(key, str);
}

function deserialize(key: string, defaultVal: any) {
  const content = localStorage.getItem(key);
  if (!content) return defaultVal;
  try {
    const value = JSON.parse(content);
    return value;
  } catch (e) {
    console.error(e);
    return defaultVal;
  }
}

function override(obj: any, toObj: any) {
  if (typeof obj === 'object' && obj) {
    for (let key of Object.keys(obj)) {
      toObj[key] = obj[key];
    }
  }
}
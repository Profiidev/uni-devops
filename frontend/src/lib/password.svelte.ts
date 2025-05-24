import type JSEncrypt from 'jsencrypt';
import {
  ContentType,
  ResponseType,
  RequestError,
  get,
  post
} from '$lib/util.svelte';
import { browser } from '$app/environment';

let encrypt: false | undefined | JSEncrypt = $state(browser && undefined);

export const getEncrypt = () => {
  return encrypt;
};

export const fetch_key = async () => {
  if (encrypt === false) {
    return RequestError.Other;
  }

  let key = await get<{ key: string }>('/auth/password/key', ResponseType.Json);

  if (typeof key !== 'object') {
    return key;
  }

  const JSEncrypt = (await import('jsencrypt')).JSEncrypt;

  encrypt = new JSEncrypt({ default_key_size: '4096' });
  encrypt.setPublicKey(key.key);
};
fetch_key();

export const password_login = async (email: string, password: string) => {
  if (!encrypt) {
    return RequestError.Other;
  }

  let encrypted_password = encrypt.encrypt(password);
  let res = await post<undefined>(
    '/auth/password/authenticate',
    ResponseType.None,
    ContentType.Json,
    JSON.stringify({
      email,
      password: encrypted_password
    })
  );

  if (typeof res !== 'object') {
    if (res === RequestError.Unauthorized) {
      fetch_key();
    }
    return res;
  }
};

export const password_register = async (
  email: string,
  password: string,
  name: string
) => {
  if (!encrypt) {
    return RequestError.Other;
  }
  let encrypted_password = encrypt.encrypt(password);
  let res = await post<undefined>(
    '/user/register',
    ResponseType.None,
    ContentType.Json,
    JSON.stringify({
      email,
      password: encrypted_password,
      name
    })
  );
  if (typeof res !== 'object') {
    if (res === RequestError.Unauthorized) {
      fetch_key();
    }
    return res;
  }
  return res;
};

#!/usr/bin/env python3

import os
from typing import Callable, TypeVar

import requests
from pydantic import BaseModel, TypeAdapter

import pve.version
import pve.nodes

ModelType = TypeVar("ModelType", bound=BaseModel | None)

AuthMethod = Callable[[requests.Session], requests.Session]


def token_auth_method(
    username: str,
    realm: str,
    token_name: str,
    secret: str,
) -> AuthMethod:
    def f(session: requests.Session):
        session.headers.update(
            {
                "Authorization": f"PVEAPIToken={username}@{realm}!{token_name}={secret}",
            }
        )
        return session

    return f


class Client:
    req: requests.Session
    base_url: str

    def __init__(
        self, base_url: str, auth_method: AuthMethod, *, tls_verify: bool = True
    ):
        self.req = requests.Session()
        self.req.verify = tls_verify
        self.req = auth_method(self.req)
        self.base_url = base_url.rstrip("/")

    def get(
        self, path: str, query: BaseModel = None, parse_as: ModelType = None
    ) -> ModelType:
        resp = self.req.get(
            f"{self.base_url}/{path}",
            json=query
            and query.model_dump_json(
                exclude_none=True,
            ),
        )
        resp.raise_for_status()
        if parse_as is None:
            return
        return TypeAdapter(parse_as).validate_python(resp.json()["data"])

    def post(
        self, path: str, body: BaseModel = None, parse_as: ModelType = None
    ) -> ModelType:
        resp = self.req.post(
            f"{self.base_url}/{path}",
            json=body
            and body.model_dump_json(
                exclude_none=True,
            ),
        )
        resp.raise_for_status()
        if parse_as is None:
            return
        return TypeAdapter(parse_as).validate_python(resp.json()["data"])

    def put(
        self, path: str, body: BaseModel = None, parse_as: ModelType = None
    ) -> ModelType:
        resp = self.req.put(
            f"{self.base_url}/{path}",
            json=body
            and body.model_dump_json(
                exclude_none=True,
            ),
        )
        resp.raise_for_status()
        if parse_as is None:
            return
        return TypeAdapter(parse_as).validate_python(resp.json()["data"])

    def delete(
        self, path: str, body: BaseModel = None, parse_as: ModelType = None
    ) -> ModelType:
        resp = self.req.delete(
            f"{self.base_url}/{path}",
            json=body
            and body.model_dump_json(
                exclude_none=True,
            ),
        )
        resp.raise_for_status()
        if parse_as is None:
            return
        return TypeAdapter(parse_as).validate_python(resp.json()["data"])


def main():
    url = os.environ["PROXMOX_URL"]
    username = os.environ["PROXMOX_USERNAME"]
    realm = os.environ["PROXMOX_REALM"]
    token_name = os.environ["PROXMOX_TOKEN_NAME"]
    token_secret = os.environ["PROXMOX_TOKEN_SECRET"]

    client = Client(
        url,
        token_auth_method(username, realm, token_name, token_secret),
    )

    version = pve.version.VersionClient(client)
    print(version.get())


if __name__ == "__main__":
    main()

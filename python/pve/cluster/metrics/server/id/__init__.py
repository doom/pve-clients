from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # An API path prefix inserted between '<host>:<port>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy.
    api_path_prefix: Optional[str] = Field(alias="api-path-prefix", default=None)
    # The InfluxDB bucket/db. Only necessary when using the http v2 api.
    bucket: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Flag to disable the plugin.
    disable: Optional[bool] = Field(default=None)
    influxdbproto: Optional[str] = Field(default=None)
    # InfluxDB max-body-size in bytes. Requests are batched up to this size.
    max_body_size: Optional[int] = Field(alias="max-body-size", default=None)
    # MTU for metrics transmission over UDP
    mtu: Optional[int] = Field(default=None)
    # The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api.
    organization: Optional[str] = Field(default=None)
    # root graphite path (ex: proxmox.mycluster.mykey)
    path: Optional[str] = Field(default=None)
    # server network port
    port: int
    # Protocol to send graphite data. TCP or UDP (default)
    proto: Optional[str] = Field(default=None)
    # server dns name or IP address
    server: str
    # graphite TCP socket timeout (default=1)
    timeout: Optional[int] = Field(default=None)
    # The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead.
    token: Optional[str] = Field(default=None)
    # Set to 0 to disable certificate verification for https endpoints.
    verify_certificate: Optional[bool] = Field(alias="verify-certificate", default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # An API path prefix inserted between '<host>:<port>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy.
    api_path_prefix: Optional[str] = Field(alias="api-path-prefix", default=None)
    # The InfluxDB bucket/db. Only necessary when using the http v2 api.
    bucket: Optional[str] = Field(default=None)
    # Flag to disable the plugin.
    disable: Optional[bool] = Field(default=None)
    influxdbproto: Optional[str] = Field(default=None)
    # InfluxDB max-body-size in bytes. Requests are batched up to this size.
    max_body_size: Optional[int] = Field(alias="max-body-size", default=None)
    # MTU for metrics transmission over UDP
    mtu: Optional[int] = Field(default=None)
    # The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api.
    organization: Optional[str] = Field(default=None)
    # root graphite path (ex: proxmox.mycluster.mykey)
    path: Optional[str] = Field(default=None)
    # server network port
    port: int
    # Protocol to send graphite data. TCP or UDP (default)
    proto: Optional[str] = Field(default=None)
    # server dns name or IP address
    server: str
    # graphite TCP socket timeout (default=1)
    timeout: Optional[int] = Field(default=None)
    # The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead.
    token: Optional[str] = Field(default=None)
    # Plugin type.
    type: str
    # Set to 0 to disable certificate verification for https endpoints.
    verify_certificate: Optional[bool] = Field(alias="verify-certificate", default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self):
        """
        Remove Metric server.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read metric server configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Create a new external metric server config
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Update metric server configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self):
        """
        Remove Metric server.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read metric server configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Create a new external metric server config
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Update metric server configuration.
        """
        return await self.client.put(self.path, parameters)

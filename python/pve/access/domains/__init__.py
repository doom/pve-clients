from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import realm as _realm


class PostParameters(BaseModel):
    # Specifies the Authentication Context Class Reference values that theAuthorization Server is being requested to use for the Auth Request.
    acr_values: Optional[str] = Field(alias="acr-values", default=None)
    # Automatically create users if they do not exist.
    autocreate: Optional[bool] = Field(default=None)
    # LDAP base domain name
    base_dn: Optional[str] = Field(default=None)
    # LDAP bind domain name
    bind_dn: Optional[str] = Field(default=None)
    # Path to the CA certificate store
    capath: Optional[str] = Field(default=None)
    # username is case-sensitive
    case_sensitive: Optional[bool] = Field(alias="case-sensitive", default=None)
    # Path to the client certificate
    cert: Optional[str] = Field(default=None)
    # Path to the client certificate key
    certkey: Optional[str] = Field(default=None)
    # OpenID Client ID
    client_id: Optional[str] = Field(alias="client-id", default=None)
    # OpenID Client Key
    client_key: Optional[str] = Field(alias="client-key", default=None)
    # Description.
    comment: Optional[str] = Field(default=None)
    # Use this as default realm
    default: Optional[bool] = Field(default=None)
    # AD domain name
    domain: Optional[str] = Field(default=None)
    # LDAP filter for user sync.
    filter: Optional[str] = Field(default=None)
    # The objectclasses for groups.
    group_classes: Optional[str] = Field(default=None)
    # LDAP base domain name for group sync. If not set, the base_dn will be used.
    group_dn: Optional[str] = Field(default=None)
    # LDAP filter for group sync.
    group_filter: Optional[str] = Field(default=None)
    # LDAP attribute representing a groups name. If not set or found, the first value of the DN will be used as name.
    group_name_attr: Optional[str] = Field(default=None)
    # OpenID Issuer Url
    issuer_url: Optional[str] = Field(alias="issuer-url", default=None)
    # LDAP protocol mode.
    mode: Optional[str] = Field(default=None)
    # LDAP bind password. Will be stored in '/etc/pve/priv/realm/<REALM>.pw'.
    password: Optional[str] = Field(default=None)
    # Server port.
    port: Optional[int] = Field(default=None)
    # Specifies whether the Authorization Server prompts the End-User for reauthentication and consent.
    prompt: Optional[str] = Field(default=None)
    # Authentication domain ID
    realm: str
    # Specifies the scopes (user details) that should be authorized and returned, for example 'email' or 'profile'.
    scopes: Optional[str] = Field(default=None)
    # Use secure LDAPS protocol. DEPRECATED: use 'mode' instead.
    secure: Optional[bool] = Field(default=None)
    # Server IP address (or DNS name)
    server1: Optional[str] = Field(default=None)
    # Fallback Server IP address (or DNS name)
    server2: Optional[str] = Field(default=None)
    # LDAPS TLS/SSL version. It's not recommended to use version older than 1.2!
    sslversion: Optional[str] = Field(default=None)
    # The default options for behavior of synchronizations.
    sync_defaults_options: Optional[str] = Field(
        alias="sync-defaults-options", default=None
    )
    # Comma separated list of key=value pairs for specifying which LDAP attributes map to which PVE user field. For example, to map the LDAP attribute 'mail' to PVEs 'email', write  'email=mail'. By default, each PVE user field is represented  by an LDAP attribute of the same name.
    sync_attributes: Optional[str] = Field(default=None)
    # Use Two-factor authentication.
    tfa: Optional[str] = Field(default=None)
    # Realm type.
    type: str
    # LDAP user attribute name
    user_attr: Optional[str] = Field(default=None)
    # The objectclasses for users.
    user_classes: Optional[str] = Field(default=None)
    # OpenID claim used to generate the unique username.
    username_claim: Optional[str] = Field(alias="username-claim", default=None)
    # Verify the server's SSL certificate
    verify: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # A comment. The GUI use this text when you select a domain (Realm) on the login window.
    comment: Optional[str] = Field(default=None)
    realm: str
    # Two-factor authentication provider.
    tfa: Optional[str] = Field(default=None)
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DomainsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'domains'}"

    def realm(self, realm: str) -> _realm.RealmClient:
        return _realm.RealmClient(
            self.client,
            self.path,
            realm,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Authentication domain index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Add an authentication server.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncDomainsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'domains'}"

    def realm(self, realm: str) -> _realm.AsyncRealmClient:
        return _realm.AsyncRealmClient(
            self.client,
            self.path,
            realm,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Authentication domain index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Add an authentication server.
        """
        return await self.client.post(self.path, parameters)

from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import cfg as _cfg
from . import cmd_safety as _cmd_safety
from . import config as _config
from . import configdb as _configdb
from . import crush as _crush
from . import fs as _fs
from . import init as _init
from . import log as _log
from . import mds as _mds
from . import mgr as _mgr
from . import mon as _mon
from . import osd as _osd
from . import pool as _pool
from . import pools as _pools
from . import restart as _restart
from . import rules as _rules
from . import start as _start
from . import status as _status
from . import stop as _stop


class GetResponseItem(BaseModel):
    pass


@dataclass
class CephClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ceph'}"

    def cfg(self) -> _cfg.CfgClient:
        return _cfg.CfgClient(
            self.client,
            self.path,
        )

    def osd(self) -> _osd.OsdClient:
        return _osd.OsdClient(
            self.client,
            self.path,
        )

    def mds(self) -> _mds.MdsClient:
        return _mds.MdsClient(
            self.client,
            self.path,
        )

    def mgr(self) -> _mgr.MgrClient:
        return _mgr.MgrClient(
            self.client,
            self.path,
        )

    def mon(self) -> _mon.MonClient:
        return _mon.MonClient(
            self.client,
            self.path,
        )

    def fs(self) -> _fs.FsClient:
        return _fs.FsClient(
            self.client,
            self.path,
        )

    def pool(self) -> _pool.PoolClient:
        return _pool.PoolClient(
            self.client,
            self.path,
        )

    def pools(self) -> _pools.PoolsClient:
        return _pools.PoolsClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.ConfigClient:
        return _config.ConfigClient(
            self.client,
            self.path,
        )

    def configdb(self) -> _configdb.ConfigdbClient:
        return _configdb.ConfigdbClient(
            self.client,
            self.path,
        )

    def init(self) -> _init.InitClient:
        return _init.InitClient(
            self.client,
            self.path,
        )

    def stop(self) -> _stop.StopClient:
        return _stop.StopClient(
            self.client,
            self.path,
        )

    def start(self) -> _start.StartClient:
        return _start.StartClient(
            self.client,
            self.path,
        )

    def restart(self) -> _restart.RestartClient:
        return _restart.RestartClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def crush(self) -> _crush.CrushClient:
        return _crush.CrushClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.LogClient:
        return _log.LogClient(
            self.client,
            self.path,
        )

    def rules(self) -> _rules.RulesClient:
        return _rules.RulesClient(
            self.client,
            self.path,
        )

    def cmd_safety(self) -> _cmd_safety.CmdSafetyClient:
        return _cmd_safety.CmdSafetyClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCephClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ceph'}"

    def cfg(self) -> _cfg.AsyncCfgClient:
        return _cfg.AsyncCfgClient(
            self.client,
            self.path,
        )

    def osd(self) -> _osd.AsyncOsdClient:
        return _osd.AsyncOsdClient(
            self.client,
            self.path,
        )

    def mds(self) -> _mds.AsyncMdsClient:
        return _mds.AsyncMdsClient(
            self.client,
            self.path,
        )

    def mgr(self) -> _mgr.AsyncMgrClient:
        return _mgr.AsyncMgrClient(
            self.client,
            self.path,
        )

    def mon(self) -> _mon.AsyncMonClient:
        return _mon.AsyncMonClient(
            self.client,
            self.path,
        )

    def fs(self) -> _fs.AsyncFsClient:
        return _fs.AsyncFsClient(
            self.client,
            self.path,
        )

    def pool(self) -> _pool.AsyncPoolClient:
        return _pool.AsyncPoolClient(
            self.client,
            self.path,
        )

    def pools(self) -> _pools.AsyncPoolsClient:
        return _pools.AsyncPoolsClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.AsyncConfigClient:
        return _config.AsyncConfigClient(
            self.client,
            self.path,
        )

    def configdb(self) -> _configdb.AsyncConfigdbClient:
        return _configdb.AsyncConfigdbClient(
            self.client,
            self.path,
        )

    def init(self) -> _init.AsyncInitClient:
        return _init.AsyncInitClient(
            self.client,
            self.path,
        )

    def stop(self) -> _stop.AsyncStopClient:
        return _stop.AsyncStopClient(
            self.client,
            self.path,
        )

    def start(self) -> _start.AsyncStartClient:
        return _start.AsyncStartClient(
            self.client,
            self.path,
        )

    def restart(self) -> _restart.AsyncRestartClient:
        return _restart.AsyncRestartClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def crush(self) -> _crush.AsyncCrushClient:
        return _crush.AsyncCrushClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.AsyncLogClient:
        return _log.AsyncLogClient(
            self.client,
            self.path,
        )

    def rules(self) -> _rules.AsyncRulesClient:
        return _rules.AsyncRulesClient(
            self.client,
            self.path,
        )

    def cmd_safety(self) -> _cmd_safety.AsyncCmdSafetyClient:
        return _cmd_safety.AsyncCmdSafetyClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

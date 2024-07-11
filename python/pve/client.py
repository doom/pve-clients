from typing import Protocol, TypeVar

from pydantic import BaseModel

Parsable = int | str | BaseModel | None | list["Parsable"] | dict[str, "Parsable"]

ModelType = TypeVar("ModelType", bound=Parsable)


class AbstractClient(Protocol):
    def get(
        self, path: str, query: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    def post(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    def put(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    def delete(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass


class AsyncAbstractClient(Protocol):
    async def get(
        self, path: str, query: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    async def post(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    async def put(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

    async def delete(
        self, path: str, body: BaseModel = None, parse_as: type[ModelType] = None
    ) -> ModelType:
        pass

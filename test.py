#!/usr/bin/env python3

import uuid
from uuid import UUID

import requests

BASE_URL = "http://127.0.0.1:2137"


def format_url(uuid: UUID):
    return f"{BASE_URL}/user/{uuid}"


def create_user(user) -> UUID:
    return UUID(requests.post(f"{BASE_URL}/user", json=user).json())


def get_user(uuid: UUID):
    return requests.get(format_url(uuid)).json()


def update_user(uuid: UUID, patch):
    assert requests.patch(format_url(uuid), json=patch).status_code == 200


def delete_user(uuid: UUID):
    assert requests.delete(format_url(uuid)).status_code == 200


if __name__ == "__main__":
    user = {"first_name": "Andrzej", "last_name": "Głębocki", "balance": 1_000_000}
    patch_user = {"first_name": "Ignacy"}

    uuid = create_user(user)
    assert get_user(uuid) == user
    update_user(uuid, patch_user)
    user["first_name"] = patch_user["first_name"]
    assert get_user(uuid) == user
    delete_user(uuid)
    try:
        get_user(uuid)
        print("Error: user should be deleted, but isn't")
    except Exception as e:
        pass

    print("Test finished.")

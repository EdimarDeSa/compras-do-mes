from typing import Literal, Optional

from django.contrib.auth.hashers import make_password
from rest_framework import serializers
from rest_framework.authtoken.views import ObtainAuthToken

from .models import *
from .password_policy import validate_password

__all__ = [
    "UsersSerializer",
    "DefaultCategorysSerializer",
    "UnityTypesSerializer",
    "UserCategorysSerializer",
    "DefaltProductsSerializer",
    "MarketSerializer",
    "UserProductsSerializer",
    "ShoppingListSerializer",
    "ShoppingLogSerializer",
    "ProductListSerializer",
]


class UsersSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = Users
        fields = serializers.ALL_FIELDS
        extra_kwargs = {"password": {"write_only": True}}

    def validate_password(self, value: str) -> str:
        validate_password(value)

        return value

    def create(self, validated_data: dict[Literal["password"], str]) -> Users:
        password = validated_data.pop("password")
        new = dict()
        new["password"] = make_password(password)
        validated_data.update(new)

        user = Users(**validated_data)
        user.save()
        return user


class DefaultCategorysSerializer(serializers.ModelSerializer):
    class Meta:
        model = DefaultCategorys
        fields = serializers.ALL_FIELDS


class UnityTypesSerializer(serializers.ModelSerializer):
    class Meta:
        model = UnityTypes
        fields = serializers.ALL_FIELDS


class DefaltProductsSerializer(serializers.ModelSerializer):
    class Meta:
        model = DefaultProducts
        fields = serializers.ALL_FIELDS


class MarketSerializer(serializers.ModelSerializer):
    class Meta:
        model = Market
        fields = serializers.ALL_FIELDS


class UserCategorysSerializer(serializers.ModelSerializer):
    class Meta:
        model = UserCategorys
        fields = serializers.ALL_FIELDS


class UserProductsSerializer(serializers.ModelSerializer):
    class Meta:
        model = UserProducts
        fields = serializers.ALL_FIELDS


class ShoppingListSerializer(serializers.ModelSerializer):
    class Meta:
        model = ShoppingList
        fields = serializers.ALL_FIELDS


class ShoppingLogSerializer(serializers.ModelSerializer):
    class Meta:
        model = ShoppingLog
        fields = serializers.ALL_FIELDS


class ProductListSerializer(serializers.ModelSerializer):
    class Meta:
        model = ProductList
        fields = serializers.ALL_FIELDS

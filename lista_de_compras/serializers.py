from rest_framework import serializers
from rest_framework.exceptions import ValidationError
from djoser.serializers import UserCreateSerializer

from .models import *
from .password_policy import PasswordPolicy

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


class UsersSerializer(UserCreateSerializer):
    class Meta(UserCreateSerializer.Meta):
        model = Users
        fields = serializers.ALL_FIELDS
        extra_kwargs = {
            "password": {
                "write_only": True,
                "required": True,
            },
            "username": {"required": True},
            "email": {"required": True},
            "last_login": {"read_only": True},
            "is_superuser": {"read_only": True},
            "is_staff": {"read_only": True},
            "is_active": {"read_only": True},
            "date_joined": {"read_only": True},
            "groups": {"read_only": True},
            "user_permissions": {"read_only": True},
        }

    def create(self, validated_data: dict[str, str]) -> Users:
        policy = PasswordPolicy(validated_data["password"])
        policy.validate()

        if not policy.is_valid:
            raise ValidationError(policy.errors, "invalid_password")

        user = Users.objects.create_user(**validated_data)
        return user


class UserPropertySerializer(serializers.ModelSerializer):
    class Meta:
        abstract = True

    order_by = "name"

    user_id = serializers.StringRelatedField()


class MarketSerializer(UserPropertySerializer):
    class Meta:
        model = Market
        fields = serializers.ALL_FIELDS

    def create(self, validated_data: dict[str, any]) -> Market:
        market = super().create(validated_data)
        return Market.objects.get(_id=market.pk)


class UserCategorysSerializer(UserPropertySerializer):
    class Meta:
        model = UserCategorys
        fields = serializers.ALL_FIELDS


class UserProductsSerializer(UserPropertySerializer):
    class Meta:
        model = UserProducts
        fields = serializers.ALL_FIELDS

    unity_types_id = serializers.StringRelatedField()
    price_unity_types_id = serializers.StringRelatedField()
    category_id = serializers.StringRelatedField()


class ShoppingListSerializer(UserPropertySerializer):
    class Meta:
        model = ShoppingList
        fields = serializers.ALL_FIELDS


class ShoppingLogSerializer(UserPropertySerializer):
    class Meta:
        model = ShoppingLog
        fields = serializers.ALL_FIELDS


class ProductListSerializer(serializers.ModelSerializer):
    class Meta:
        model = ProductList
        fields = serializers.ALL_FIELDS

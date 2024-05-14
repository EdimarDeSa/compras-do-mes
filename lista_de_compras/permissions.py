from rest_framework.permissions import (
    AllowAny,
    IsAuthenticated,
    BasePermission,
    IsAdminUser,
)
from rest_framework.request import Request
from rest_framework.viewsets import ModelViewSet

__all__ = ["AllowAny", "IsAuthenticated", "IsAdminUser", "IsSelf"]


class IsSelf(BasePermission):
    """
    Custom permission to only allow users to view or edit their own information.
    """

    def has_permission(self, request: Request, view: ModelViewSet):
        print("request.user: ", request.user)
        print("request.user.is_authenticated: ", request.user.is_authenticated)
        print("request.auth: ", request.auth)
        print("request.stream: ", request.stream)
        print("request.data: ", request.successful_authenticator)

        return request.user.is_authenticated

    def has_object_permission(self, request: Request, view: ModelViewSet, obj):
        return obj == request.user and request.user.is_authenticated

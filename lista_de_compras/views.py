from rest_framework import viewsets
from rest_framework.permissions import IsAuthenticated, AllowAny
from rest_framework.response import Response

from .filters import DefaultProductsFilter
from .models import *
from .serializers import *


# Create your views here.
class UserViewSet(viewsets.ModelViewSet):
    queryset = Users.objects.all().order_by("nickname")
    serializer_class = UsersSerializer

    def get_permissions(self):
        permission_classes = []
        if self.action == "create":
            permission_classes = [AllowAny]
        else:
            permission_classes = [IsAuthenticated]
        return [permission() for permission in permission_classes]


class DefaultCategorysViewSet(viewsets.ModelViewSet):
    queryset = DefaultCategorys.objects.all().order_by("name")
    serializer_class = DefaultCategorysSerializer


class UnityTypesViewSet(viewsets.ModelViewSet):
    queryset = UnityTypes.objects.all().order_by("name")
    serializer_class = UnityTypesSerializer
    search_fields = ["name"]


class DefaultProductsViewSet(viewsets.ModelViewSet):
    queryset = DefaultProducts.objects.all().order_by("name")
    serializer_class = DefaltProductsSerializer
    filterset_class = DefaultProductsFilter


class MarketViewSet(viewsets.ModelViewSet):
    queryset = Market.objects.all().order_by("name")
    serializer_class = MarketSerializer


class UserCategorysViewSet(viewsets.ModelViewSet):
    queryset = UserCategorys.objects.all().order_by("name")
    serializer_class = UserCategorysSerializer


class UserProductsViewSet(viewsets.ModelViewSet):
    queryset = UserProducts.objects.all().order_by("name")
    serializer_class = UserProductsSerializer


class ShoppingListViewSet(viewsets.ModelViewSet):
    queryset = ShoppingList.objects.all().order_by("name")
    serializer_class = ShoppingListSerializer


class ShoppingLogViewSet(viewsets.ModelViewSet):
    queryset = ShoppingLog.objects.all().order_by("buy_date")
    serializer_class = ShoppingLogSerializer


class ProductListViewSet(viewsets.ModelViewSet):
    queryset = ProductList.objects.all().order_by("user_product_id")
    serializer_class = ProductListSerializer


class ForgotPasswordViewSet(viewsets.ViewSet):
    def create(self, request):
        return Response({"message": "Forgot Password"})


class ResetPasswordViewSet(viewsets.ViewSet):
    def create(self, request):
        return Response({"message": "Reset Password"})

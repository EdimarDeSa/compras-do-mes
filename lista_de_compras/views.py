from rest_framework import viewsets
from rest_framework.response import Response
from rest_framework.authentication import TokenAuthentication
from djoser.views import UserViewSet
from rest_framework.utils import model_meta

from .filters import DefaultProductsFilter
from .models import *
from .serializers import *
from .permissions import *


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


# Create your views here.
class UsersViewSet(UserViewSet):
    authentication_classes = [TokenAuthentication]
    queryset = Users.objects.all().order_by("username")
    serializer_class = UsersSerializer

    def get_permissions(self):
        permission_classes = []
        if self.action == "create":
            permission_classes = [AllowAny]
        else:
            permission_classes = [IsSelf]
        return [permission() for permission in permission_classes]


class UserPropertyViewSet(viewsets.ModelViewSet):
    class Meta:
        abstract = True

    order_by = "name"

    def get_queryset(self):
        if str(self.request.user) == "AnonymousUser":
            return self.serializer_class.Meta.model.objects.none()

        if self.request.user.is_superuser:
            return self.serializer_class.Meta.model.objects.all().order_by(
                self.order_by
            )

        return self.serializer_class.Meta.model.objects.filter(
            user_id=self.request.user
        ).order_by(self.order_by)

    def list(self, request, *args, **kwargs):
        queryset = self.get_queryset()
        if not queryset:
            return Response(
                {"message": "Você precisa estar logado para acessar essa página"}
            )
        serializer = self.get_serializer(queryset, many=True)
        return Response(serializer.data)

    def create(self, request, *args, **kwargs):
        data = {field: value for field, value in request.data.items()}
        data["user_id"] = request.user

        serializer = self.serializer_class(data=data)
        market_data = serializer.create(data)
        return Response(self.serializer_class(market_data).data)


class MarketViewSet(UserPropertyViewSet):
    serializer_class = MarketSerializer


class UserCategorysViewSet(UserPropertyViewSet):
    serializer_class = UserCategorysSerializer


class UserProductsViewSet(UserPropertyViewSet):
    serializer_class = UserProductsSerializer


class ShoppingListViewSet(UserPropertyViewSet):
    serializer_class = ShoppingListSerializer


class ShoppingLogViewSet(UserPropertyViewSet):
    order_by = "buy_date"
    serializer_class = ShoppingLogSerializer


class ProductListViewSet(UserPropertyViewSet):
    order_by = "user_product_id"
    serializer_class = ProductListSerializer

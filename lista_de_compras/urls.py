from django.urls import include, path
from rest_framework import routers

from . import views

router = routers.DefaultRouter()
router.register(r"users", views.UserViewSet, basename="users")
router.register(
    r"default_categorys",
    views.DefaultCategorysViewSet,
    basename="default_categorys",
)
router.register(r"unity_types", views.UnityTypesViewSet, basename="unity_types")
router.register(
    r"default_products",
    views.DefaultProductsViewSet,
    basename="default_products",
)
router.register(r"market", views.MarketViewSet, basename="market")
router.register(
    r"user_categorys", views.UserCategorysViewSet, basename="user_categorys"
)
router.register(
    r"user_products", views.UserProductsViewSet, basename="user_products"
)
router.register(
    r"shopping_list", views.ShoppingListViewSet, basename="shopping_list"
)
router.register(
    r"shopping_log", views.ShoppingLogViewSet, basename="shopping_log"
)
router.register(
    r"product_list", views.ProductListViewSet, basename="product_list"
)

urlpatterns = [
    path("", include(router.urls)),
]

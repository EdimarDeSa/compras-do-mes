from django.db.models.signals import post_migrate
from django.dispatch import receiver

from .models import DefaultCategorys, DefaultProducts, UnityTypes


@receiver(post_migrate)
def create_default_categorys(sender, **kwargs):
    if sender.label == "lista_de_compras":
        if DefaultCategorys.objects.all().count() == 0:
            DefaultCategorys.objects.create(name="Frios")
            DefaultCategorys.objects.create(name="Laticíneos")
            DefaultCategorys.objects.create(name="Carnes")
            DefaultCategorys.objects.create(name="Condimentos")
            DefaultCategorys.objects.create(name="Hortifruti")
            DefaultCategorys.objects.create(name="Bazar")
            DefaultCategorys.objects.create(name="Bebidas")
            DefaultCategorys.objects.create(name="Limpeza")
            DefaultCategorys.objects.create(name="Higiene Pessoal")
            DefaultCategorys.objects.create(name="Padaria")
            DefaultCategorys.objects.create(name="Enlatados")
            DefaultCategorys.objects.create(name="Outros")


@receiver(post_migrate)
def create_unity_types(sender, **kwargs):
    if sender.label == "lista_de_compras":
        if UnityTypes.objects.all().count() == 0:
            UnityTypes.objects.create(name="Gramas", abbreviation="g", factor=1)
            UnityTypes.objects.create(name="Kilos", abbreviation="kg", factor=1000)
            UnityTypes.objects.create(name="Pacote", abbreviation="pac", factor=1)
            UnityTypes.objects.create(name="Unidade", abbreviation="und", factor=1)
            UnityTypes.objects.create(name="Litros", abbreviation="l", factor=1000)
            UnityTypes.objects.create(name="Mililitros", abbreviation="ml", factor=1)
            UnityTypes.objects.create(name="Dúzia", abbreviation="dz", factor=12)
            UnityTypes.objects.create(name="Caixa", abbreviation="cx", factor=1)
            UnityTypes.objects.create(name="Bandeja", abbreviation="bdj", factor=1)
            UnityTypes.objects.create(name="Galão", abbreviation="gal", factor=3785)


@receiver(post_migrate)
def create_default_products(sender, **kwargs):
    if sender.label == "lista_de_compras":
        if DefaultProducts.objects.all().count() == 0:
            DefaultProducts.objects.create(
                name="Manga",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Banana",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Arroz",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Feijão",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Carne",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Frango",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Café",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Açúcar",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Sal",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Óleo",
                unity_types_id=UnityTypes.objects.get(name="Litros"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Leite",
                unity_types_id=UnityTypes.objects.get(name="Litros"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Queijo",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Presunto",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Pão",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Padaria"),
            )
            DefaultProducts.objects.create(
                name="Macarrão",
                unity_types_id=UnityTypes.objects.get(name="Pacote"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Molho de tomate",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Enlatados"),
            )
            DefaultProducts.objects.create(
                name="Farinha de trigo",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Ovos",
                unity_types_id=UnityTypes.objects.get(name="Caixa"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Maçã",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Laranja",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Cebola",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Alho",
                unity_types_id=UnityTypes.objects.get(name="Gramas"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Batata",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Tomate",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Hortifruti"),
            )
            DefaultProducts.objects.create(
                name="Suco",
                unity_types_id=UnityTypes.objects.get(name="Litros"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Refrigerante",
                unity_types_id=UnityTypes.objects.get(name="Litros"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Suco de pozinho",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Água",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Água com gáz",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Sabonete",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Shampoo",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Condicionador",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Papel higiênico",
                unity_types_id=UnityTypes.objects.get(name="Pacote"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Detergente",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Amaciante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Desinfetante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Alvejante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Limpador multiuso",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Creme dental",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Escova de dentes",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Enxaguante bucal",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Desodorante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(
                    name="Higiene Pessoal"
                ),
            )
            DefaultProducts.objects.create(
                name="Sabão em pó",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Esponja de cozinha",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Limpeza"),
            )
            DefaultProducts.objects.create(
                name="Leite condensado",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Creme de leite",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Gelatina",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Chá",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Vinagre",
                unity_types_id=UnityTypes.objects.get(name="Litros"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Mostarda",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Ketchup",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Molho de pimenta",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Condimentos"),
            )
            DefaultProducts.objects.create(
                name="Salsicha",
                unity_types_id=UnityTypes.objects.get(name="Pacote"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Linguiça",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Peixe",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Atum",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Sardinha",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Carnes"),
            )
            DefaultProducts.objects.create(
                name="Margarina",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Manteiga",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Iogurte",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Geléia",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Cereal",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Granola",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Aveia",
                unity_types_id=UnityTypes.objects.get(name="Kilos"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Biscoitos",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Bolacha",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Chocolate",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Achocolatado",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Balas",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Chiclete",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Pipoca",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Amendoim",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Castanhas",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Nozes",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Pistache",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Cerveja",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Vinho",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Espumante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Vodka",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Whisky",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Tequila",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Rum",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Martini",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Campari",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Licor",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bebidas"),
            )
            DefaultProducts.objects.create(
                name="Azeitonas",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Enlatados"),
            )
            DefaultProducts.objects.create(
                name="Picles",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Enlatados"),
            )
            DefaultProducts.objects.create(
                name="Frutas em calda",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Enlatados"),
            )
            DefaultProducts.objects.create(
                name="Geleia real",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Mel",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Creme de avelã",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Pasta de amendoin",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Marshmallows",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Açúcar mascavo",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Adoçante",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Melado",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Chantilly",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Bazar"),
            )
            DefaultProducts.objects.create(
                name="Patê",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Enlatados"),
            )
            DefaultProducts.objects.create(
                name="Ricota",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )
            DefaultProducts.objects.create(
                name="Requeijão",
                unity_types_id=UnityTypes.objects.get(name="Unidade"),
                default_categorys_id=DefaultCategorys.objects.get(name="Laticíneos"),
            )

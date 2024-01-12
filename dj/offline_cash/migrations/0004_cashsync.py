# Generated by Django 4.2.2 on 2023-10-26 18:18

from django.db import migrations, models


class Migration(migrations.Migration):
    dependencies = [
        ("offline_cash", "0003_offlinecashqueue"),
    ]

    operations = [
        migrations.CreateModel(
            name="CashSync",
            fields=[
                (
                    "id",
                    models.BigAutoField(
                        auto_created=True,
                        primary_key=True,
                        serialize=False,
                        verbose_name="ID",
                    ),
                ),
                ("created_on", models.DateTimeField(auto_now_add=True)),
                ("updated_on", models.DateTimeField(auto_now=True)),
                ("service_agreement_id", models.CharField(max_length=256)),
                ("previous_tx_hash", models.CharField(max_length=256)),
                ("amount", models.CharField(max_length=128)),
                ("from_did", models.CharField(max_length=256)),
                ("to_did", models.CharField(max_length=256)),
                ("issued_at", models.BigIntegerField()),
                ("expiry_at", models.BigIntegerField()),
                ("tx_hash", models.CharField(max_length=256)),
                ("signed_tx_hash", models.CharField(max_length=256)),
                ("has_claimed", models.CharField(max_length=128)),
            ],
            options={
                "db_table": "cash_sync",
            },
        ),
    ]

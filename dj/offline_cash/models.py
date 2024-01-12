from django.db import models
from django.contrib.postgres.fields import ArrayField


# Create your models here.


class DateTimeBase(models.Model):
    created_on = models.DateTimeField(auto_now_add=True)
    updated_on = models.DateTimeField(auto_now=True)

    class Meta:
        abstract = True


class OfflineCashQueue(DateTimeBase):
    from_did = models.CharField(max_length=256)
    to_did = models.CharField(max_length=256)
    certificates = models.JSONField()
    transactions = models.JSONField()
    tx_hash = models.CharField(max_length=256)
    signed_tx_hash = models.CharField(max_length=256)
    status = models.BooleanField()
    class Meta:
        db_table = "offline_cash_queue"


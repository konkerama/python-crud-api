from prometheus_client import Counter


HTTP_REQUESTED_CUSTOMERS_TOTAL = Counter(
    "http_requested_customers_total",
    "Number of times a certain customer has been requested.",
    labelnames=("customer",),
)


def inc_requested_customer(customer: str, amount: int = 1) -> None:
    """Increment the customer counter from other modules/functions."""
    if not customer:
        return
    HTTP_REQUESTED_CUSTOMERS_TOTAL.labels(customer).inc(amount)
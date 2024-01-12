# Code Sync To AWS Machine


## Using `rsync` command

### For Dev Machine

```sh
rsync -e "ssh -i ~/github/co2/offline-cash-dev.pem" -avz --exclude='.env/' --exclude='.target/' ~/github/co2/bank-network-agent ubuntu@13.232.107.160:/home/ubuntu
```
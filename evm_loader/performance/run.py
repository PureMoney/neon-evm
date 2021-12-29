from spl_ import *
from uniswap import *
from uniswap_proxy import add_liquidity_proxy, swap_proxy
from erc20 import *


parser = argparse.ArgumentParser(description='Process some integers.')
parser.add_argument('--count', metavar="count of the transaction",  type=int,  help='count transaction (>=1)', default=trx_cnt)
parser.add_argument('--step', metavar="step of the test", type=str,
                    help= ' For ERC20.transfers: deploy_erc20, create_senders, create_collateral, create_acc, create_trx, veryfy_trx'
                          ' For spl-token transfers: create_senders, create_collateral, create_acc, create_trx, verify_trx'
                          ' For swap operations: deploy_erc20, create_senders, create_acc, mint_acc, add_liquidity, swap_tokens'
                          ' To top up the senders balance: transfer_to_senders')
parser.add_argument('--postfix', metavar="filename postfix", type=str,  help='0,1,2..', default='')
parser.add_argument('--type', metavar="transfer type", type=str,  help='erc20, spl, swap', default='erc20')
parser.add_argument('--key', metavar="keypair", type=str,  help='/home/solana/collateral-pool-keypair.json', default='')
parser.add_argument('--balance', metavar="balance",  type=int,  help='only for transfer_to_senders')

args = parser.parse_args()

if args.step == "deploy_erc20":
    deploy_erc20(args)
elif args.step == "create_acc":
    if args.type == "spl":
        create_account_spl(args)
    elif args.type == "swap":
        create_account_swap(args)
    else:
        create_accounts(args)
elif args.step == "create_trx":
    if args.type == "spl":
        create_transactions_spl(args)
    elif args.type == "erc20":
        create_transactions(args)
elif args.step == "send_trx":
    send_transactions(args)
elif args.step == "create_senders":
    create_senders(args)
elif args.step == "verify_trx":
    if args.type == "spl":
        verify_trx_spl(args)
    else:
        verify_trx(args)
elif args.step == "add_liquidity":
    add_liquidity_proxy(args)
elif args.step == "create_collateral":
    create_collateral_pool(args)
elif args.step == "transfer_to_senders":
    transfer_to_senders(args)
elif args.step == "mint_acc":
    if args.type == "swap":
        mint_account_swap(args)
elif args.step == "swap_tokens":
    if args.type == "swap":
        swap_proxy(args)

{\rtf1\ansi\ansicpg1252\cocoartf2761
\cocoatextscaling0\cocoaplatform0{\fonttbl\f0\froman\fcharset0 Times-Roman;}
{\colortbl;\red255\green255\blue255;\red255\green255\blue255;\red0\green0\blue0;}
{\*\expandedcolortbl;;\cssrgb\c100000\c100000\c100000;\cssrgb\c0\c0\c0;}
\paperw11900\paperh16840\margl1440\margr1440\vieww11520\viewh8400\viewkind0
\deftab720
\pard\pardeftab720\partightenfactor0

\f0\fs30 \cf0 \cb2 \expnd0\expndtw0\kerning0
\outl0\strokewidth0 \strokec3 mod contract;\
mod msg;\
\
use cosmwasm_std::entry_point;\
use cosmwasm_std::\{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult\};\
\
#[entry_point]\
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: msg::InstantiateMsg) -> StdResult<Response> \{\
    contract::instantiate(deps, info, msg)\
\}\
\
#[entry_point]\
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> StdResult<Response> \{\
    contract::execute(deps, env, info, msg)\
\}\
\
#[entry_point]\
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> \{\
    contract::query(deps, msg)\
\}}
{\rtf1\ansi\ansicpg1252\cocoartf2761
\cocoatextscaling0\cocoaplatform0{\fonttbl\f0\froman\fcharset0 Times-Roman;}
{\colortbl;\red255\green255\blue255;\red255\green255\blue255;\red0\green0\blue0;}
{\*\expandedcolortbl;;\cssrgb\c100000\c100000\c100000;\cssrgb\c0\c0\c0;}
\paperw11900\paperh16840\margl1440\margr1440\vieww9100\viewh9860\viewkind0
\deftab720
\pard\pardeftab720\partightenfactor0

\f0\fs30 \cf0 \cb2 \expnd0\expndtw0\kerning0
\outl0\strokewidth0 \strokec3 use schemars::JsonSchema;\
use serde::\{Deserialize, Serialize\};\
\
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]\
pub struct InstantiateMsg \{\
    pub reward_token: String,\
\}\
\
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]\
#[serde(rename_all = "snake_case")]\
pub enum ExecuteMsg \{\
    RegisterReferral \{ code: String \},\
    ClaimReferral \{ user_to_claim: String \},\
\}\
\
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]\
#[serde(rename_all = "snake_case")]\
pub enum QueryMsg \{\
    GetReferralCode \{ user: String \},\
\}}
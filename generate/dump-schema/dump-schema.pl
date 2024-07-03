#!/usr/bin/perl

# Source: https://git.proxmox.com/?p=pve-docs.git;a=blob;f=extractapi.pl

use strict;
use warnings;

use JSON;
use PVE::API2;
use PVE::RESTHandler;

my $tree = PVE::RESTHandler::api_dump_remove_refs(PVE::RESTHandler::api_dump('PVE::API2'));

print to_json($tree, {pretty => 1, canonical => 1});

exit(0);

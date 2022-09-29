// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

import React from "react";
import {
  CircularProgress,
  CircularProgressLabel,
  VStack,
  Skeleton,
  Stack,
  Text,
  Flex,
  Box,
} from "@chakra-ui/react";
import { Column, DataTable, LinkCell } from "./DataTable";
import { FaStop } from "react-icons/fa";
import { GrDocumentDownload } from "react-icons/gr";
import fileDownload from "js-file-download";

export enum QueryStatus {
  QUEUED = "QUEUED",
  RUNNING = "RUNNING",
  FAILED = "FAILED",
  COMPLETED = "COMPLETED",
}

export interface Query {
  job_id: string;
  status: QueryStatus;
  num_stages: number;
  percent_complete: number;
}

export interface QueriesListProps {
  queries?: Query[];
}

export const ActionsCell: (props: any) => React.ReactNode = (props: any) => {
  const handleDownload = (url: string, filename: string) => {
    fetch(url, {
      method: "GET",
      headers: {
        Accept: "application/json",
      },
    }).then(async (res) => {
      fileDownload(await res.arrayBuffer(), filename);
    });
  };
  return (
    <Flex>
      <FaStop color={"red"} title={"Stop this job"} />
      <Box mx={2}></Box>
      <button
        onClick={() => {
          handleDownload(
            "/api/job/" + props.value + "/dot",
            props.value + ".dot"
          );
        }}
      >
        <GrDocumentDownload title={"Download DOT Plan"} />
      </button>
    </Flex>
  );
};

export const ProgressCell: (props: any) => React.ReactNode = (props: any) => {
  return (
    <CircularProgress value={props.value} color="orange.400">
      <CircularProgressLabel>{props.value}%</CircularProgressLabel>
    </CircularProgress>
  );
};

const columns: Column<any>[] = [
  {
    Header: "Job ID",
    accessor: "job_id",
    Cell: LinkCell,
  },
  {
    Header: "Status",
    accessor: "job_status",
  },
  {
    Header: "Number of Stages",
    accessor: "num_stages",
  },
  {
    Header: "Progress",
    accessor: "percent_complete",
    Cell: ProgressCell,
  },
  {
    Header: "Actions",
    accessor: "job_id",
    id: "action_cell",
    Cell: ActionsCell,
  },
];

const getSkeletion = () => (
  <>
    <Skeleton height={5} />
    <Skeleton height={5} />
    <Skeleton height={5} />
    <Skeleton height={5} />
    <Skeleton height={5} />
  </>
);

export const QueriesList: React.FunctionComponent<QueriesListProps> = ({
  queries,
}) => {
  const isLoaded = typeof queries !== "undefined";

  return (
    <VStack flex={1} p={4} w={"100%"} alignItems={"flex-start"}>
      <Text mb={4}>Queries</Text>
      <Stack w={"100%"} flex={1}>
        {isLoaded ? (
          <DataTable
            columns={columns}
            data={queries || []}
            pageSize={10}
            pb={10}
          />
        ) : (
          getSkeletion()
        )}
      </Stack>
    </VStack>
  );
};

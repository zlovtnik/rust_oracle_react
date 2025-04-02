import React, { useState } from 'react';
import { Table, Button, Space, Popconfirm } from 'antd';
import type { TableProps } from 'antd';
import ItemForm from '../components/ItemForm';
import { NFeIdentification } from '../types/nfeTypes';

const initialNFeData: NFeIdentification[] = [
    {
        internalKey: '1',
        cUF: '35',
        cNF: '12345678',
        natOp: 'Venda de Mercadoria',
        mod: '55',
        serie: '1',
        nNF: '1001',
        dhEmi: new Date().toISOString(),
        tpNF: '1',
        idDest: '1',
        cMunFG: '3550308',
        tpImp: '1',
        tpEmis: '1',
        cDV: '9',
        tpAmb: '2',
        finNFe: '1',
        indFinal: '0',
        indPres: '1',
        procEmi: '0',
        verProc: 'MySystem V1.0',
    },
    {
        internalKey: '2',
        cUF: '41',
        cNF: '87654321',
        natOp: 'Remessa para Conserto',
        mod: '55',
        serie: '1',
        nNF: '1002',
        dhEmi: new Date(Date.now() - 86400000).toISOString(),
        tpNF: '0',
        idDest: '2',
        cMunFG: '4106902',
        tpImp: '1',
        tpEmis: '1',
        cDV: '5',
        tpAmb: '2',
        finNFe: '1',
        indFinal: '0',
        indPres: '0',
        procEmi: '0',
        verProc: 'MySystem V1.0',
    },
];

type NFeIdeFormData = Pick<NFeIdentification,
    'nNF' | 'serie' | 'dhEmi' | 'natOp' | 'tpNF'
>;

const IdePage: React.FC = () => {
    const [items, setItems] = useState<NFeIdentification[]>(initialNFeData);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isEditing, setIsEditing] = useState(false);
    const [editingItem, setEditingItem] = useState<NFeIdentification | null>(null);

    const handleAdd = () => {
        setIsEditing(false);
        setEditingItem(null);
        setIsModalOpen(true);
    };

    const handleEdit = (record: NFeIdentification) => {
        setIsEditing(true);
        setEditingItem(record);
        setIsModalOpen(true);
    };

    const handleDelete = (internalKey: string) => {
        setItems(currentItems => currentItems.filter(item => item.internalKey !== internalKey));
    };

    const handleModalCancel = () => {
        setIsModalOpen(false);
    };

    const handleModalFinish = (values: NFeIdeFormData) => {
        if (isEditing && editingItem) {
            setItems(currentItems =>
                currentItems.map(item =>
                    item.internalKey === editingItem.internalKey
                        ? { ...editingItem, ...values }
                        : item
                )
            );
        } else {
            const newItem: NFeIdentification = {
                internalKey: Date.now().toString(),
                ...values,
                cUF: '',
                cNF: '',
                mod: '55',
                idDest: '',
                cMunFG: '',
                tpImp: '1',
                tpEmis: '1',
                cDV: '',
                tpAmb: '2',
                finNFe: '1',
                indFinal: '0',
                indPres: '0',
                procEmi: '0',
                verProc: 'MySystem V1.0',
            };
            setItems(currentItems => [...currentItems, newItem]);
        }
        setIsModalOpen(false);
    };

    const columns: TableProps<NFeIdentification>['columns'] = [
        {
            title: 'Number (nNF)',
            dataIndex: 'nNF',
            key: 'nNF',
        },
        {
            title: 'Series (serie)',
            dataIndex: 'serie',
            key: 'serie',
        },
        {
            title: 'Emission (dhEmi)',
            dataIndex: 'dhEmi',
            key: 'dhEmi',
            render: (text: string) => new Date(text).toLocaleString(),
        },
        {
            title: 'Nature (natOp)',
            dataIndex: 'natOp',
            key: 'natOp',
            ellipsis: true,
        },
        {
            title: 'Type (tpNF)',
            dataIndex: 'tpNF',
            key: 'tpNF',
            render: (text: string) => text === '0' ? 'Entrada' : 'Saída',
        },
        {
            title: 'Action',
            key: 'action',
            render: (_, record) => (
                <Space size="middle">
                    <Button type="link" onClick={() => handleEdit(record)}>Edit</Button>
                    <Popconfirm
                        title="Delete NFe Identification"
                        description="Are you sure to delete this record?"
                        onConfirm={() => handleDelete(record.internalKey)}
                        okText="Yes"
                        cancelText="No"
                    >
                        <Button type="link" danger>Delete</Button>
                    </Popconfirm>
                </Space>
            ),
        },
    ];

    return (
        <>
            <Button type="primary" onClick={handleAdd} style={{ marginBottom: 16 }}>
                Add NFe Identification
            </Button>
            <Table columns={columns} dataSource={items} rowKey="internalKey" />
            <ItemForm
                open={isModalOpen}
                isEditing={isEditing}
                initialValues={editingItem
                    ? (({ nNF, serie, dhEmi, natOp, tpNF }) => ({ nNF, serie, dhEmi, natOp, tpNF }))(editingItem)
                    : undefined}
                onFinish={handleModalFinish}
                onCancel={handleModalCancel}
            />
        </>
    );
};

export default IdePage; 
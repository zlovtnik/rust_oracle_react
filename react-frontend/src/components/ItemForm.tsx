import React, { useEffect } from 'react';
import { Modal, Form, Input, DatePicker, Select } from 'antd';
import { NFeIdentification } from '../types/nfeTypes';
import dayjs from 'dayjs';

const { Option } = Select;

type NFeIdeFormData = Pick<NFeIdentification,
    'nNF' | 'serie' | 'dhEmi' | 'natOp' | 'tpNF'
>;

interface ItemFormProps {
    open: boolean;
    isEditing: boolean;
    initialValues?: NFeIdeFormData;
    onFinish: (values: NFeIdeFormData) => void;
    onCancel: () => void;
}

const ItemForm: React.FC<ItemFormProps> = ({ open, isEditing, initialValues, onFinish, onCancel }) => {
    const [form] = Form.useForm();

    useEffect(() => {
        if (open) {
            form.resetFields();
            if (isEditing && initialValues) {
                form.setFieldsValue({
                    ...initialValues,
                    dhEmi: initialValues.dhEmi ? dayjs(initialValues.dhEmi) : null,
                });
            } else {
                form.setFieldsValue({
                    nNF: '',
                    serie: '',
                    dhEmi: null,
                    natOp: '',
                    tpNF: undefined,
                });
            }
        }
    }, [open, isEditing, initialValues, form]);

    const handleOk = () => {
        form
            .validateFields()
            .then(values => {
                const processedValues: NFeIdeFormData = {
                    ...(values as NFeIdeFormData),
                    dhEmi: values.dhEmi ? values.dhEmi.toISOString() : '',
                };
                onFinish(processedValues);
            })
            .catch(info => {
                console.log('Validate Failed:', info);
            });
    };

    return (
        <Modal
            title={isEditing ? "Edit NFe Identification" : "Add NFe Identification"}
            open={open}
            onOk={handleOk}
            onCancel={onCancel}
            okText={isEditing ? "Save" : "Create"}
            cancelText="Cancel"
            destroyOnClose
            width={600}
        >
            <Form
                form={form}
                layout="vertical"
                name="nfe_ide_form"
            >
                <Form.Item
                    name="nNF"
                    label="Number (nNF)"
                    rules={[{ required: true, message: 'Please input the document number!' }]}
                >
                    <Input />
                </Form.Item>
                <Form.Item
                    name="serie"
                    label="Series (serie)"
                    rules={[{ required: true, message: 'Please input the document series!' }]}
                >
                    <Input />
                </Form.Item>
                <Form.Item
                    name="dhEmi"
                    label="Emission Date/Time (dhEmi)"
                    rules={[{ required: true, message: 'Please select the emission date and time!' }]}
                >
                    <DatePicker showTime format="YYYY-MM-DD HH:mm:ss" style={{ width: '100%' }} />
                </Form.Item>
                <Form.Item
                    name="natOp"
                    label="Nature of Operation (natOp)"
                    rules={[{ required: true, message: 'Please input the nature of operation!' }]}
                >
                    <Input.TextArea rows={2} />
                </Form.Item>
                <Form.Item
                    name="tpNF"
                    label="Type (tpNF)"
                    rules={[{ required: true, message: 'Please select the document type!' }]}
                >
                    <Select placeholder="Select type">
                        <Option value="0">0 - Entrada</Option>
                        <Option value="1">1 - Saída</Option>
                    </Select>
                </Form.Item>
            </Form>
        </Modal>
    );
};

export default ItemForm; 
/* generated by rust_qt_binding_generator */
#ifndef TEST_OBJECT_TYPES_RUST_H
#define TEST_OBJECT_TYPES_RUST_H

#include <QObject>
#include <QVariant>
#include <QAbstractItemModel>

class ObjectInterface;
class Object : public QObject
{
    Q_OBJECT
    ObjectInterface * const d;
    Q_PROPERTY(QVariant value READ value WRITE setValue NOTIFY valueChanged FINAL)
public:
    explicit Object(QObject *parent = nullptr);
    ~Object();
    QVariant value() const;
    void setValue(const QVariant& v);
signals:
    void valueChanged();
private:
    QVariant m_value;
};
#endif // TEST_OBJECT_TYPES_RUST_H
